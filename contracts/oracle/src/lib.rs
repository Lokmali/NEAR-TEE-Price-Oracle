use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use near_sdk::json_types::U128;
use std::collections::HashMap;

// Price precision: 8 decimals (e.g., $50,000.12345678)
const PRICE_DECIMALS: u128 = 100_000_000;
const MIN_ORACLE_NODES: usize = 3;
const MAX_PRICE_AGE_NS: u64 = 300_000_000_000; // 5 minutes in nanoseconds
const MAX_PRICE_DEVIATION_PERCENT: u128 = 20; // 20% max deviation from median

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Oracle {
    /// Contract owner
    pub owner: AccountId,
    /// Paused state
    pub paused: bool,
    /// Registered oracle nodes with TEE attestation
    pub nodes: UnorderedMap<AccountId, OracleNode>,
    /// Active node list
    pub active_nodes: UnorderedSet<AccountId>,
    /// Price data for each asset
    pub prices: UnorderedMap<String, AssetPrice>,
    /// Supported assets
    pub supported_assets: UnorderedSet<String>,
    /// Minimum number of sources required
    pub min_sources: u8,
    /// Admin accounts with special permissions
    pub admins: UnorderedSet<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OracleNode {
    pub account_id: AccountId,
    pub tee_type: String, // "sgx" or "phala"
    pub attestation: String, // Base64 encoded attestation
    pub region: String,
    pub endpoint: String,
    pub registered_at: u64,
    pub last_update: u64,
    pub total_updates: u64,
    pub is_active: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetPrice {
    pub symbol: String,
    pub price: U128, // Price with PRICE_DECIMALS precision
    pub timestamp: u64,
    pub sources: Vec<PriceSource>,
    pub confidence: u8, // 0-100
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceSource {
    pub name: String,
    pub price: U128,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PriceUpdate {
    pub symbol: String,
    pub price: U128,
    pub sources: Vec<PriceSource>,
}

#[near_bindgen]
impl Oracle {
    #[init]
    pub fn new(owner: AccountId, min_sources: u8) -> Self {
        let mut admins = UnorderedSet::new(b"a");
        admins.insert(&owner);
        
        Self {
            owner: owner.clone(),
            paused: false,
            nodes: UnorderedMap::new(b"n"),
            active_nodes: UnorderedSet::new(b"an"),
            prices: UnorderedMap::new(b"p"),
            supported_assets: UnorderedSet::new(b"s"),
            min_sources,
            admins,
        }
    }

    // ==================== Node Management ====================

    /// Register a new oracle node with TEE attestation
    #[payable]
    pub fn register_node(
        &mut self,
        node_id: AccountId,
        tee_type: String,
        attestation: String,
        region: String,
        endpoint: String,
    ) {
        self.assert_admin();
        self.assert_not_paused();

        // Verify TEE attestation (simplified - in production, verify signature)
        assert!(
            tee_type == "sgx" || tee_type == "phala",
            "Invalid TEE type. Must be 'sgx' or 'phala'"
        );

        let node = OracleNode {
            account_id: node_id.clone(),
            tee_type,
            attestation,
            region,
            endpoint,
            registered_at: env::block_timestamp(),
            last_update: 0,
            total_updates: 0,
            is_active: true,
        };

        self.nodes.insert(&node_id, &node);
        self.active_nodes.insert(&node_id);

        env::log_str(&format!("Node registered: {}", node_id));
    }

    /// Deactivate an oracle node
    pub fn deactivate_node(&mut self, node_id: AccountId) {
        self.assert_admin();
        
        if let Some(mut node) = self.nodes.get(&node_id) {
            node.is_active = false;
            self.nodes.insert(&node_id, &node);
            self.active_nodes.remove(&node_id);
            env::log_str(&format!("Node deactivated: {}", node_id));
        }
    }

    /// Reactivate a node
    pub fn activate_node(&mut self, node_id: AccountId) {
        self.assert_admin();
        
        if let Some(mut node) = self.nodes.get(&node_id) {
            node.is_active = true;
            self.nodes.insert(&node_id, &node);
            self.active_nodes.insert(&node_id);
            env::log_str(&format!("Node activated: {}", node_id));
        }
    }

    // ==================== Asset Management ====================

    /// Add a new supported asset
    pub fn add_asset(&mut self, symbol: String) {
        self.assert_admin();
        self.supported_assets.insert(&symbol);
        env::log_str(&format!("Asset added: {}", symbol));
    }

    /// Remove an asset
    pub fn remove_asset(&mut self, symbol: String) {
        self.assert_admin();
        self.supported_assets.remove(&symbol);
        self.prices.remove(&symbol);
        env::log_str(&format!("Asset removed: {}", symbol));
    }

    // ==================== Price Updates ====================

    /// Submit price update from oracle node
    pub fn submit_price(&mut self, updates: Vec<PriceUpdate>) {
        self.assert_not_paused();
        
        let sender = env::predecessor_account_id();
        
        // Verify sender is an active node
        let mut node = self.nodes.get(&sender).expect("Node not registered");
        assert!(node.is_active, "Node is not active");

        for update in updates {
            // Verify asset is supported
            assert!(
                self.supported_assets.contains(&update.symbol),
                "Asset not supported: {}",
                update.symbol
            );

            // Verify minimum sources
            assert!(
                update.sources.len() >= self.min_sources as usize,
                "Insufficient price sources"
            );

            // Calculate median price and validate
            let validated_price = self.validate_and_aggregate_price(&update);

            let asset_price = AssetPrice {
                symbol: update.symbol.clone(),
                price: validated_price,
                timestamp: env::block_timestamp(),
                sources: update.sources,
                confidence: self.calculate_confidence(&update),
            };

            self.prices.insert(&update.symbol, &asset_price);
        }

        // Update node stats
        node.last_update = env::block_timestamp();
        node.total_updates += 1;
        self.nodes.insert(&sender, &node);

        env::log_str(&format!("Price update from node: {}", sender));
    }

    /// Validate and aggregate price from multiple sources
    fn validate_and_aggregate_price(&self, update: &PriceUpdate) -> U128 {
        let mut prices: Vec<u128> = update.sources.iter()
            .map(|s| s.price.0)
            .collect();
        
        // Sort to find median
        prices.sort();
        
        let median = if prices.len() % 2 == 0 {
            (prices[prices.len() / 2 - 1] + prices[prices.len() / 2]) / 2
        } else {
            prices[prices.len() / 2]
        };

        // Validate no price deviates too much from median
        for price in &prices {
            let deviation = if *price > median {
                (*price - median) * 100 / median
            } else {
                (median - *price) * 100 / median
            };
            
            assert!(
                deviation <= MAX_PRICE_DEVIATION_PERCENT,
                "Price deviation too high"
            );
        }

        U128(median)
    }

    /// Calculate confidence score based on source agreement
    fn calculate_confidence(&self, update: &PriceUpdate) -> u8 {
        let prices: Vec<u128> = update.sources.iter().map(|s| s.price.0).collect();
        let avg = prices.iter().sum::<u128>() / prices.len() as u128;
        
        let variance: u128 = prices.iter()
            .map(|p| {
                let diff = if *p > avg { *p - avg } else { avg - *p };
                diff * diff
            })
            .sum::<u128>() / prices.len() as u128;
        
        // Lower variance = higher confidence
        let std_dev_percent = ((variance as f64).sqrt() / avg as f64) * 100.0;
        
        if std_dev_percent < 1.0 {
            100
        } else if std_dev_percent < 2.0 {
            95
        } else if std_dev_percent < 5.0 {
            85
        } else if std_dev_percent < 10.0 {
            70
        } else {
            50
        }
    }

    // ==================== Query Methods ====================

    /// Get current price for an asset
    pub fn get_price(&self, symbol: String) -> Option<AssetPrice> {
        let price = self.prices.get(&symbol)?;
        
        // Check if price is not too old
        let age = env::block_timestamp() - price.timestamp;
        if age > MAX_PRICE_AGE_NS {
            env::log_str(&format!("Warning: Price for {} is stale", symbol));
        }
        
        Some(price)
    }

    /// Get multiple prices at once
    pub fn get_prices(&self, symbols: Vec<String>) -> Vec<Option<AssetPrice>> {
        symbols.iter().map(|s| self.get_price(s.clone())).collect()
    }

    /// Get all supported assets
    pub fn get_supported_assets(&self) -> Vec<String> {
        self.supported_assets.iter().collect()
    }

    /// Get node information
    pub fn get_node(&self, node_id: AccountId) -> Option<OracleNode> {
        self.nodes.get(&node_id)
    }

    /// Get all active nodes
    pub fn get_active_nodes(&self) -> Vec<OracleNode> {
        self.active_nodes.iter()
            .filter_map(|id| self.nodes.get(&id))
            .collect()
    }

    /// Get all nodes (active and inactive)
    pub fn get_all_nodes(&self) -> Vec<OracleNode> {
        self.nodes.values().collect()
    }

    // ==================== Shade Agent Framework Integration ====================

    /// Query price with simple text interface for AI agents
    pub fn agent_query(&self, query: String) -> String {
        // Parse simple queries like "price of BTC", "get ETH price", etc.
        let query_lower = query.to_lowercase();
        
        // Extract symbol from query
        let symbols = ["btc", "eth", "near", "bnb", "ada", "sol", "dot", "matic", "avax"];
        
        for symbol in &symbols {
            if query_lower.contains(symbol) {
                if let Some(price) = self.get_price(symbol.to_uppercase()) {
                    let price_value = price.price.0 as f64 / PRICE_DECIMALS as f64;
                    return format!(
                        "The current price of {} is ${:.2} (confidence: {}%, updated {} seconds ago)",
                        price.symbol,
                        price_value,
                        price.confidence,
                        (env::block_timestamp() - price.timestamp) / 1_000_000_000
                    );
                } else {
                    return format!("Price data for {} is not available", symbol.to_uppercase());
                }
            }
        }
        
        "I couldn't understand the query. Please ask about a specific asset like 'What is the price of BTC?'".to_string()
    }

    /// Batch query for AI agents
    pub fn agent_batch_query(&self, symbols: Vec<String>) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        
        for symbol in symbols {
            if let Some(price) = self.get_price(symbol.clone()) {
                let price_value = price.price.0 as f64 / PRICE_DECIMALS as f64;
                result.insert(symbol, price_value);
            }
        }
        
        result
    }

    // ==================== Admin Functions ====================

    /// Pause the contract
    pub fn pause(&mut self) {
        self.assert_owner();
        self.paused = true;
        env::log_str("Contract paused");
    }

    /// Resume the contract
    pub fn resume(&mut self) {
        self.assert_owner();
        self.paused = false;
        env::log_str("Contract resumed");
    }

    /// Add admin
    pub fn add_admin(&mut self, account: AccountId) {
        self.assert_owner();
        self.admins.insert(&account);
        env::log_str(&format!("Admin added: {}", account));
    }

    /// Remove admin
    pub fn remove_admin(&mut self, account: AccountId) {
        self.assert_owner();
        assert!(account != self.owner, "Cannot remove owner");
        self.admins.remove(&account);
        env::log_str(&format!("Admin removed: {}", account));
    }

    /// Update minimum sources
    pub fn set_min_sources(&mut self, min_sources: u8) {
        self.assert_admin();
        self.min_sources = min_sources;
    }

    // ==================== Internal Methods ====================

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only owner can call this method"
        );
    }

    fn assert_admin(&self) {
        let caller = env::predecessor_account_id();
        assert!(
            self.admins.contains(&caller),
            "Only admins can call this method"
        );
    }

    fn assert_not_paused(&self) {
        assert!(!self.paused, "Contract is paused");
    }
}

// ==================== Unit Tests ====================

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_initialization() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        
        let contract = Oracle::new(accounts(0), 5);
        assert_eq!(contract.owner, accounts(0));
        assert!(!contract.paused);
        assert_eq!(contract.min_sources, 5);
    }

    #[test]
    fn test_add_asset() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        
        let mut contract = Oracle::new(accounts(0), 5);
        contract.add_asset("BTC".to_string());
        
        assert!(contract.supported_assets.contains(&"BTC".to_string()));
    }

    #[test]
    fn test_register_node() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        
        let mut contract = Oracle::new(accounts(0), 5);
        
        contract.register_node(
            accounts(1),
            "sgx".to_string(),
            "mock_attestation".to_string(),
            "us-east-1".to_string(),
            "https://node1.example.com".to_string(),
        );
        
        let node = contract.get_node(accounts(1)).unwrap();
        assert_eq!(node.tee_type, "sgx");
        assert!(node.is_active);
    }
}

