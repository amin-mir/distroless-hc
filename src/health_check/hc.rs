use crate::config::Config;

use anyhow::Result;
use futures::future;
use reqwest::Client;
use tokio::time;

pub struct HealthCheck<'a> {
    pub config: &'a Config,
}

impl<'a> HealthCheck<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn check(&self) -> Result<Vec<(String, bool)>> {
        let client = Client::new();

        let futs = self.config.hosts.iter().map(|url| {
            let client = &client;

            async move {
                let res = self.make_request(client, url).await;
                Ok((url.to_string(), res))
            }
        });

        future::join_all(futs).await.into_iter().collect()
    }

    async fn make_request(&self, client: &Client, url: &str) -> bool {
        let retries = self.config.retries;

        for _ in 0..retries {
            match client.get(url).timeout(self.config.timeout).send().await {
                Ok(res) => {
                    if res.status().is_success() {
                        return true;
                    }
                }
                Err(e) => eprintln!("{} is not healthy: {}", url, e),
            }

            time::sleep(self.config.interval).await;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_healthcheck_check_healthy() {
        let server = MockServer::start().await;

        let called = AtomicUsize::new(0);
        Mock::given(method("GET"))
            .and(path("/healthcheck"))
            .respond_with(move |_: &wiremock::Request| {
                let new_called = called.fetch_add(1, Ordering::Relaxed) + 1;

                if new_called >= 4 {
                    ResponseTemplate::new(201)
                } else {
                    ResponseTemplate::new(500)
                }
            })
            .expect(4)
            .mount(&server)
            .await;

        let url = format!("{}/healthcheck", server.uri());
        let config = Config {
            hosts: vec![url.clone()],
            interval: time::Duration::from_millis(100),
            timeout: time::Duration::from_millis(100),
            retries: 4,
        };

        let hc = HealthCheck::new(&config);
        let res = hc.check().await.unwrap();
        assert_eq!(res, vec![(url, true)]);
    }

    #[tokio::test]
    async fn test_healthcheck_check_not_healthy() {
        let server = MockServer::start().await;

        let called = AtomicUsize::new(0);
        Mock::given(method("GET"))
            .and(path("/healthcheck"))
            .respond_with(move |_: &wiremock::Request| {
                let new_called = called.fetch_add(1, Ordering::Relaxed) + 1;

                if new_called >= 4 {
                    ResponseTemplate::new(201)
                } else {
                    ResponseTemplate::new(500)
                }
            })
            .expect(3)
            .mount(&server)
            .await;

        let url = format!("{}/healthcheck", server.uri());
        let config = Config {
            hosts: vec![url.clone()],
            interval: time::Duration::from_millis(100),
            timeout: time::Duration::from_millis(100),
            retries: 3,
        };

        let hc = HealthCheck::new(&config);
        let res = hc.check().await.unwrap();
        assert_eq!(res, vec![(url, false)]);
    }
}
