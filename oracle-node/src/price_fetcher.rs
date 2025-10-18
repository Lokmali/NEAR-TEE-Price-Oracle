use crate::config::PriceSourcesConfig;
use crate::types::{PriceData, PriceSource};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

pub struct PriceFetcher {
    client: Client,
    config: PriceSourcesConfig,
}

impl PriceFetcher {
    pub fn new(config: PriceSourcesConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub async fn fetch_price(&self, symbol: &str) -> Result<PriceData> {
        let mut sources = Vec::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        // Fetch from all enabled sources
        if let Ok(price) = self.fetch_coingecko(symbol).await {
            sources.push(PriceSource {
                name: "CoinGecko".to_string(),
                price,
                timestamp,
            });
        }

        if self.config.binance_enabled {
            if let Ok(price) = self.fetch_binance(symbol).await {
                sources.push(PriceSource {
                    name: "Binance".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if self.config.kraken_enabled {
            if let Ok(price) = self.fetch_kraken(symbol).await {
                sources.push(PriceSource {
                    name: "Kraken".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if let Some(_) = &self.config.coinmarketcap_api_key {
            if let Ok(price) = self.fetch_coinmarketcap(symbol).await {
                sources.push(PriceSource {
                    name: "CoinMarketCap".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if let Some(_) = &self.config.cryptocompare_api_key {
            if let Ok(price) = self.fetch_cryptocompare(symbol).await {
                sources.push(PriceSource {
                    name: "CryptoCompare".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if self.config.coinbase_enabled {
            if let Ok(price) = self.fetch_coinbase(symbol).await {
                sources.push(PriceSource {
                    name: "Coinbase".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if self.config.huobi_enabled {
            if let Ok(price) = self.fetch_huobi(symbol).await {
                sources.push(PriceSource {
                    name: "Huobi".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if self.config.kucoin_enabled {
            if let Ok(price) = self.fetch_kucoin(symbol).await {
                sources.push(PriceSource {
                    name: "KuCoin".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if self.config.bitfinex_enabled {
            if let Ok(price) = self.fetch_bitfinex(symbol).await {
                sources.push(PriceSource {
                    name: "Bitfinex".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if self.config.gemini_enabled {
            if let Ok(price) = self.fetch_gemini(symbol).await {
                sources.push(PriceSource {
                    name: "Gemini".to_string(),
                    price,
                    timestamp,
                });
            }
        }

        if sources.len() < 5 {
            return Err(anyhow!(
                "Insufficient price sources: got {}, need at least 5",
                sources.len()
            ));
        }

        let median_price = self.calculate_median(&sources);

        Ok(PriceData {
            symbol: symbol.to_string(),
            median_price,
            sources,
            timestamp,
        })
    }

    // ==================== Price Source Implementations ====================

    async fn fetch_coingecko(&self, symbol: &str) -> Result<f64> {
        let coin_id = self.symbol_to_coingecko_id(symbol);
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            coin_id
        );

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response[coin_id]["usd"]
            .as_f64()
            .ok_or_else(|| anyhow!("Failed to parse CoinGecko price"))?;

        debug!("CoinGecko price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_binance(&self, symbol: &str) -> Result<f64> {
        let pair = format!("{}USDT", symbol);
        let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response["price"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Failed to parse Binance price"))?;

        debug!("Binance price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_kraken(&self, symbol: &str) -> Result<f64> {
        let pair = self.symbol_to_kraken_pair(symbol);
        let url = format!("https://api.kraken.com/0/public/Ticker?pair={}", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let result = &response["result"];
        let pair_data = result.as_object()
            .and_then(|obj| obj.values().next())
            .ok_or_else(|| anyhow!("Failed to parse Kraken response"))?;
        
        let price = pair_data["c"][0]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Failed to parse Kraken price"))?;

        debug!("Kraken price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_coinmarketcap(&self, symbol: &str) -> Result<f64> {
        let api_key = self.config.coinmarketcap_api_key.as_ref()
            .ok_or_else(|| anyhow!("CoinMarketCap API key not configured"))?;

        let url = format!(
            "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
            symbol
        );

        let response: Value = self.client
            .get(&url)
            .header("X-CMC_PRO_API_KEY", api_key)
            .send()
            .await?
            .json()
            .await?;

        let price = response["data"][symbol]["quote"]["USD"]["price"]
            .as_f64()
            .ok_or_else(|| anyhow!("Failed to parse CoinMarketCap price"))?;

        debug!("CoinMarketCap price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_cryptocompare(&self, symbol: &str) -> Result<f64> {
        let api_key = self.config.cryptocompare_api_key.as_ref()
            .ok_or_else(|| anyhow!("CryptoCompare API key not configured"))?;

        let url = format!(
            "https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD",
            symbol
        );

        let response: Value = self.client
            .get(&url)
            .header("authorization", format!("Apikey {}", api_key))
            .send()
            .await?
            .json()
            .await?;

        let price = response["USD"]
            .as_f64()
            .ok_or_else(|| anyhow!("Failed to parse CryptoCompare price"))?;

        debug!("CryptoCompare price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_coinbase(&self, symbol: &str) -> Result<f64> {
        let pair = format!("{}-USD", symbol);
        let url = format!("https://api.coinbase.com/v2/prices/{}/spot", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response["data"]["amount"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Failed to parse Coinbase price"))?;

        debug!("Coinbase price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_huobi(&self, symbol: &str) -> Result<f64> {
        let pair = format!("{}usdt", symbol.to_lowercase());
        let url = format!("https://api.huobi.pro/market/detail/merged?symbol={}", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response["tick"]["close"]
            .as_f64()
            .ok_or_else(|| anyhow!("Failed to parse Huobi price"))?;

        debug!("Huobi price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_kucoin(&self, symbol: &str) -> Result<f64> {
        let pair = format!("{}-USDT", symbol);
        let url = format!("https://api.kucoin.com/api/v1/market/orderbook/level1?symbol={}", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response["data"]["price"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Failed to parse KuCoin price"))?;

        debug!("KuCoin price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_bitfinex(&self, symbol: &str) -> Result<f64> {
        let pair = format!("t{}USD", symbol);
        let url = format!("https://api-pub.bitfinex.com/v2/ticker/{}", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response[6]
            .as_f64()
            .ok_or_else(|| anyhow!("Failed to parse Bitfinex price"))?;

        debug!("Bitfinex price for {}: ${}", symbol, price);
        Ok(price)
    }

    async fn fetch_gemini(&self, symbol: &str) -> Result<f64> {
        let pair = format!("{}usd", symbol.to_lowercase());
        let url = format!("https://api.gemini.com/v1/pubticker/{}", pair);

        let response: Value = self.client.get(&url).send().await?.json().await?;
        
        let price = response["last"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Failed to parse Gemini price"))?;

        debug!("Gemini price for {}: ${}", symbol, price);
        Ok(price)
    }

    // ==================== Helper Methods ====================

    fn calculate_median(&self, sources: &[PriceSource]) -> f64 {
        let mut prices: Vec<f64> = sources.iter().map(|s| s.price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = prices.len();
        if len % 2 == 0 {
            (prices[len / 2 - 1] + prices[len / 2]) / 2.0
        } else {
            prices[len / 2]
        }
    }

    fn symbol_to_coingecko_id(&self, symbol: &str) -> &str {
        match symbol {
            "BTC" => "bitcoin",
            "ETH" => "ethereum",
            "NEAR" => "near",
            "BNB" => "binancecoin",
            "XRP" => "ripple",
            "ADA" => "cardano",
            "SOL" => "solana",
            "DOT" => "polkadot",
            "MATIC" => "matic-network",
            "AVAX" => "avalanche-2",
            _ => symbol,
        }
    }

    fn symbol_to_kraken_pair(&self, symbol: &str) -> String {
        match symbol {
            "BTC" => "XXBTZUSD".to_string(),
            "ETH" => "XETHZUSD".to_string(),
            "NEAR" => "NEARUSD".to_string(),
            _ => format!("{}USD", symbol),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_calculation() {
        let config = PriceSourcesConfig {
            coingecko_api_key: None,
            binance_enabled: true,
            kraken_enabled: true,
            coinmarketcap_api_key: None,
            cryptocompare_api_key: None,
            coinbase_enabled: true,
            huobi_enabled: false,
            kucoin_enabled: false,
            bitfinex_enabled: false,
            gemini_enabled: false,
        };

        let fetcher = PriceFetcher::new(config);

        let sources = vec![
            PriceSource { name: "A".to_string(), price: 100.0, timestamp: 0 },
            PriceSource { name: "B".to_string(), price: 102.0, timestamp: 0 },
            PriceSource { name: "C".to_string(), price: 98.0, timestamp: 0 },
            PriceSource { name: "D".to_string(), price: 101.0, timestamp: 0 },
            PriceSource { name: "E".to_string(), price: 99.0, timestamp: 0 },
        ];

        let median = fetcher.calculate_median(&sources);
        assert_eq!(median, 100.0);
    }
}

