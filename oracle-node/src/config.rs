use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub node_id: String,
    pub region: String,
    pub endpoint: String,
    pub update_interval: u64, // seconds
    pub assets: Vec<String>,
    pub near: NearConfig,
    pub tee: TeeConfig,
    pub price_sources: PriceSourcesConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NearConfig {
    pub network: String, // "mainnet" or "testnet"
    pub rpc_url: String,
    pub contract_id: String,
    pub signer_id: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeeConfig {
    pub tee_type: String, // "sgx" or "phala"
    pub enable_attestation: bool,
    pub attestation_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceSourcesConfig {
    pub coingecko_api_key: Option<String>,
    pub binance_enabled: bool,
    pub kraken_enabled: bool,
    pub coinmarketcap_api_key: Option<String>,
    pub cryptocompare_api_key: Option<String>,
    pub coinbase_enabled: bool,
    pub huobi_enabled: bool,
    pub kucoin_enabled: bool,
    pub bitfinex_enabled: bool,
    pub gemini_enabled: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

