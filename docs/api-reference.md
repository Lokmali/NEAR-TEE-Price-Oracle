# API Reference

Complete API reference for the NEAR TEE Oracle smart contract.

## Contract Address

- **Mainnet**: `oracle.near`
- **Testnet**: `oracle.testnet`

## View Methods (Read-Only)

### `get_price`

Get the current price for a single asset.

```rust
pub fn get_price(&self, symbol: String) -> Option<AssetPrice>
```

**Parameters:**
- `symbol` (String): Asset symbol (e.g., "BTC", "ETH")

**Returns:**
```json
{
  "symbol": "BTC",
  "price": "6723456000000",
  "timestamp": 1234567890000000000,
  "sources": [
    {
      "name": "Binance",
      "price": "6723450000000",
      "timestamp": "1234567890000000000"
    }
  ],
  "confidence": 98
}
```

**Example:**
```bash
near view oracle.near get_price '{"symbol": "BTC"}'
```

---

### `get_prices`

Get prices for multiple assets.

```rust
pub fn get_prices(&self, symbols: Vec<String>) -> Vec<Option<AssetPrice>>
```

**Parameters:**
- `symbols` (Vec<String>): Array of asset symbols

**Returns:** Array of AssetPrice objects

**Example:**
```bash
near view oracle.near get_prices '{"symbols": ["BTC", "ETH", "NEAR"]}'
```

---

### `get_supported_assets`

Get list of all supported assets.

```rust
pub fn get_supported_assets(&self) -> Vec<String>
```

**Returns:** Array of asset symbols

**Example:**
```bash
near view oracle.near get_supported_assets '{}'
```

---

### `get_node`

Get information about a specific oracle node.

```rust
pub fn get_node(&self, node_id: AccountId) -> Option<OracleNode>
```

**Parameters:**
- `node_id` (AccountId): Node's NEAR account ID

**Returns:**
```json
{
  "account_id": "node1.near",
  "tee_type": "sgx",
  "attestation": "base64_encoded_attestation",
  "region": "us-east-1",
  "endpoint": "https://node1.oracle.near",
  "registered_at": 1234567890000000000,
  "last_update": 1234567890000000000,
  "total_updates": 15234,
  "is_active": true
}
```

---

### `get_active_nodes`

Get all active oracle nodes.

```rust
pub fn get_active_nodes(&self) -> Vec<OracleNode>
```

**Returns:** Array of OracleNode objects

---

### `get_all_nodes`

Get all nodes (active and inactive).

```rust
pub fn get_all_nodes(&self) -> Vec<OracleNode>
```

**Returns:** Array of OracleNode objects

---

### `agent_query`

Natural language query interface for AI agents.

```rust
pub fn agent_query(&self, query: String) -> String
```

**Parameters:**
- `query` (String): Natural language question

**Returns:** Human-readable string response

**Example:**
```bash
near view oracle.near agent_query '{"query": "What is the price of Bitcoin?"}'
```

**Response:**
```
"The current price of BTC is $67,234.56 (confidence: 98%, updated 45 seconds ago)"
```

---

### `agent_batch_query`

Batch query for AI agents.

```rust
pub fn agent_batch_query(&self, symbols: Vec<String>) -> HashMap<String, f64>
```

**Parameters:**
- `symbols` (Vec<String>): Array of asset symbols

**Returns:** Map of symbol to price (as float)

## Change Methods (Write)

### `submit_price`

Submit price update from oracle node.

```rust
pub fn submit_price(&mut self, updates: Vec<PriceUpdate>)
```

**Permissions:** Registered and active oracle nodes only

**Parameters:**
```json
{
  "updates": [
    {
      "symbol": "BTC",
      "price": "6723456000000",
      "sources": [
        {
          "name": "Binance",
          "price": "6723450000000",
          "timestamp": "1234567890000000000"
        }
      ]
    }
  ]
}
```

**Gas:** ~30 TGas per update

---

### Admin Methods

#### `register_node`

Register a new oracle node.

```rust
pub fn register_node(
    &mut self,
    node_id: AccountId,
    tee_type: String,
    attestation: String,
    region: String,
    endpoint: String,
)
```

**Permissions:** Admin only

**Parameters:**
- `node_id`: NEAR account ID for the node
- `tee_type`: "sgx" or "phala"
- `attestation`: Base64-encoded TEE attestation
- `region`: Geographic region (e.g., "us-east-1")
- `endpoint`: Node's public API endpoint

---

#### `deactivate_node`

Deactivate an oracle node.

```rust
pub fn deactivate_node(&mut self, node_id: AccountId)
```

**Permissions:** Admin only

---

#### `activate_node`

Reactivate a node.

```rust
pub fn activate_node(&mut self, node_id: AccountId)
```

**Permissions:** Admin only

---

#### `add_asset`

Add a new supported asset.

```rust
pub fn add_asset(&mut self, symbol: String)
```

**Permissions:** Admin only

---

#### `remove_asset`

Remove an asset.

```rust
pub fn remove_asset(&mut self, symbol: String)
```

**Permissions:** Admin only

---

#### `pause`

Pause the contract.

```rust
pub fn pause(&mut self)
```

**Permissions:** Owner only

---

#### `resume`

Resume the contract.

```rust
pub fn resume(&mut self)
```

**Permissions:** Owner only

---

#### `add_admin`

Add an admin account.

```rust
pub fn add_admin(&mut self, account: AccountId)
```

**Permissions:** Owner only

---

#### `remove_admin`

Remove an admin account.

```rust
pub fn remove_admin(&mut self, account: AccountId)
```

**Permissions:** Owner only

---

#### `set_min_sources`

Update minimum sources requirement.

```rust
pub fn set_min_sources(&mut self, min_sources: u8)
```

**Permissions:** Admin only

## Data Types

### AssetPrice

```rust
pub struct AssetPrice {
    pub symbol: String,
    pub price: U128,        // Price with 8 decimal precision
    pub timestamp: u64,     // Nanoseconds since epoch
    pub sources: Vec<PriceSource>,
    pub confidence: u8,     // 0-100
}
```

### PriceSource

```rust
pub struct PriceSource {
    pub name: String,
    pub price: U128,
    pub timestamp: u64,
}
```

### OracleNode

```rust
pub struct OracleNode {
    pub account_id: AccountId,
    pub tee_type: String,
    pub attestation: String,
    pub region: String,
    pub endpoint: String,
    pub registered_at: u64,
    pub last_update: u64,
    pub total_updates: u64,
    pub is_active: bool,
}
```

### PriceUpdate

```rust
pub struct PriceUpdate {
    pub symbol: String,
    pub price: U128,
    pub sources: Vec<PriceSource>,
}
```

## Constants

```rust
const PRICE_DECIMALS: u128 = 100_000_000;  // 8 decimal places
const MIN_ORACLE_NODES: usize = 3;
const MAX_PRICE_AGE_NS: u64 = 300_000_000_000;  // 5 minutes
const MAX_PRICE_DEVIATION_PERCENT: u128 = 20;  // 20%
```

## Error Handling

The contract will panic with descriptive messages for:
- Unauthorized access
- Invalid asset symbols
- Insufficient price sources
- Excessive price deviation
- Paused contract state

## Gas Costs

Typical gas costs:
- `get_price`: < 1 TGas (view call, free)
- `get_prices`: < 1 TGas per asset (view call, free)
- `submit_price`: ~30 TGas per update
- `register_node`: ~5 TGas
- `add_asset`: ~2 TGas

## Rate Limits

No explicit rate limits on view calls. For change methods, standard NEAR network limits apply.

## Best Practices

1. **Always check for null**: `get_price()` can return `None`
2. **Validate timestamps**: Ensure price isn't too old
3. **Check confidence**: Use prices with >90% confidence
4. **Batch queries**: Use `get_prices()` for multiple assets
5. **Handle errors**: Implement proper error handling

## Examples

See [Developer Guide](developer-guide.md) for complete integration examples in:
- Rust (NEAR contracts)
- JavaScript/TypeScript
- Python

## Support

- Documentation: https://docs.near-tee-oracle.io
- GitHub: https://github.com/near-tee-oracle
- Discord: [Community Channel]

