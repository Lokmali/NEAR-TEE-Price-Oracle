use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub median_price: f64,
    pub sources: Vec<PriceSource>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSource {
    pub name: String,
    pub price: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub symbol: String,
    pub price: String, // U128 as string
    pub sources: Vec<NearPriceSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearPriceSource {
    pub name: String,
    pub price: String, // U128 as string
    pub timestamp: String, // u64 as string
}

impl PriceData {
    pub fn to_near_format(&self) -> PriceUpdate {
        const PRICE_DECIMALS: u128 = 100_000_000;
        
        PriceUpdate {
            symbol: self.symbol.clone(),
            price: ((self.median_price * PRICE_DECIMALS as f64) as u128).to_string(),
            sources: self.sources.iter().map(|s| NearPriceSource {
                name: s.name.clone(),
                price: ((s.price * PRICE_DECIMALS as f64) as u128).to_string(),
                timestamp: s.timestamp.to_string(),
            }).collect(),
        }
    }
}

