mod config;
mod price_fetcher;
mod tee;
mod publisher;
mod types;

use anyhow::Result;
use config::Config;
use price_fetcher::PriceFetcher;
use publisher::NearPublisher;
use tee::TeeAttestor;
use tracing::{info, error};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("oracle_node=info")
        .init();

    info!("Starting NEAR TEE Oracle Node...");

    // Load configuration
    let config = Config::load("config/config.toml")?;
    info!("Configuration loaded successfully");

    // Initialize TEE attestor
    let tee_attestor = TeeAttestor::new(&config.tee)?;
    let attestation = tee_attestor.generate_attestation().await?;
    info!("TEE attestation generated: {}", &attestation[..20]);

    // Initialize price fetcher
    let price_fetcher = PriceFetcher::new(config.price_sources.clone());
    
    // Initialize NEAR publisher
    let publisher = NearPublisher::new(&config.near).await?;
    info!("Connected to NEAR network: {}", config.near.network);

    // Main loop
    let mut interval = time::interval(Duration::from_secs(config.update_interval));
    
    info!("Oracle node is running. Press Ctrl+C to stop.");
    
    loop {
        interval.tick().await;
        
        match run_update_cycle(&price_fetcher, &publisher, &config).await {
            Ok(_) => info!("Price update cycle completed successfully"),
            Err(e) => error!("Error in update cycle: {}", e),
        }
    }
}

async fn run_update_cycle(
    fetcher: &PriceFetcher,
    publisher: &NearPublisher,
    config: &Config,
) -> Result<()> {
    info!("Starting price update cycle...");

    // Fetch prices for all configured assets
    let mut price_updates = Vec::new();

    for asset in &config.assets {
        info!("Fetching price for {}...", asset);
        
        match fetcher.fetch_price(asset).await {
            Ok(price_data) => {
                info!(
                    "Fetched {} price: ${:.2} from {} sources",
                    asset,
                    price_data.median_price,
                    price_data.sources.len()
                );
                price_updates.push(price_data);
            }
            Err(e) => {
                error!("Failed to fetch price for {}: {}", asset, e);
            }
        }
    }

    // Publish to NEAR blockchain
    if !price_updates.is_empty() {
        publisher.publish_prices(price_updates).await?;
        info!("Published {} price updates to NEAR", price_updates.len());
    }

    Ok(())
}

