mod config;
use config::Config;

mod health_check;
use health_check::HealthCheck;

use anyhow::{anyhow, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::from_env()?;
    println!("{:?}", cfg);

    let hc = HealthCheck::new(&cfg);
    let results = hc.check().await?;

    // Collect unhealthy hosts into a Vec
    let unhealthy_hosts: Vec<String> = results
        .into_iter()
        .filter_map(|(host, healthy)| if !healthy { Some(host) } else { None })
        .collect();

    // Use `if` as an expression to simplify the final result
    if unhealthy_hosts.is_empty() {
        println!("All hosts are healthy");
        Ok(())
    } else {
        Err(anyhow!("Unhealthy hosts: {:?}", unhealthy_hosts))
    }
}
