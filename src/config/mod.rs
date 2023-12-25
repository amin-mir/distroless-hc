use std::env;
use std::time::Duration;

use anyhow::{Context, Result};

mod dur;
use dur::TimeUnit;

const DEFAULT_TIMEOUT: &str = "1s";
const DEFAULT_RETRIES: &str = "5";
const DEFAULT_INTERVAL: &str = "1s";

#[derive(Debug)]
pub struct Config {
    pub hosts: Vec<String>,
    pub timeout: Duration,
    pub retries: usize,
    pub interval: Duration,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let hosts = env::var("HOSTS").context("Missing HOSTS environment variable")?;

        let timeout: TimeUnit = env::var("TIMEOUT")
            .unwrap_or(DEFAULT_TIMEOUT.to_string())
            .parse()
            .context("TIMEOUT is not a valid number")?;

        let retries = env::var("RETRIES")
            .unwrap_or(DEFAULT_RETRIES.to_string())
            .parse()
            .context("RETRIES is not a valid number")?;

        let interval: TimeUnit = env::var("INTERVAL")
            .unwrap_or(DEFAULT_INTERVAL.to_string())
            .parse()
            .context("INTERVAL is not a valid number")?;

        Ok(Config {
            hosts: hosts.split(',').map(|s| s.trim().to_string()).collect(),
            timeout: timeout.into(),
            retries,
            interval: interval.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_hosts_required() {
        temp_env::with_var("HOSTS", None::<&str>, || {
            let config = Config::from_env();
            assert!(config.is_err());
        });
    }

    #[test]
    fn test_config_from_env_default_values() {
        temp_env::with_var("HOSTS", Some("localhost"), || {
            let config = Config::from_env().unwrap();
            assert_eq!(config.hosts, vec!["localhost".to_string()]);
            assert_eq!(config.timeout, Duration::from_millis(1000));
            assert_eq!(config.retries, 5);
            assert_eq!(config.interval, Duration::from_millis(1000));
        });
    }

    #[test]
    fn test_config_from_env_custom_values() {
        temp_env::with_vars(
            [
                ("HOSTS", Some(" host1, host2, host3")),
                ("TIMEOUT", Some("500ms")),
                ("RETRIES", Some("3")),
                ("INTERVAL", Some("2s")),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(
                    config.hosts,
                    vec![
                        "host1".to_string(),
                        "host2".to_string(),
                        "host3".to_string()
                    ]
                );
                assert_eq!(config.timeout, Duration::from_millis(500));
                assert_eq!(config.retries, 3);
                assert_eq!(config.interval, Duration::from_secs(2));
            },
        );
    }
}
