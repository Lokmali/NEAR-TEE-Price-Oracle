# Developer Guide

Welcome to the NEAR TEE Oracle developer guide. This document will help you integrate the oracle into your applications.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Smart Contract Integration](#smart-contract-integration)
3. [JavaScript/TypeScript Integration](#javascripttypescript-integration)
4. [Python Integration](#python-integration)
5. [API Reference](#api-reference)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

- NEAR account (testnet or mainnet)
- NEAR CLI installed
- Node.js 18+ or Python 3.8+
- Basic understanding of blockchain oracles

### Quick Start

1. **Install NEAR CLI**
```bash
npm install -g near-cli
```

2. **Create NEAR Account** (if you don't have one)
```bash
near create-account your-app.testnet --useFaucet
```

3. **Test Oracle Connection**
```bash
near view oracle.testnet get_price '{"symbol": "BTC"}'
```

## Smart Contract Integration

### Reading Prices in Your NEAR Contract

```rust
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, Gas, Promise};
use near_sdk::json_types::U128;

#[ext_contract(ext_oracle)]
pub trait OracleContract {
    fn get_price(&self, symbol: String) -> Option<AssetPrice>;
    fn get_prices(&self, symbols: Vec<String>) -> Vec<Option<AssetPrice>>;
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AssetPrice {
    pub symbol: String,
    pub price: U128,
    pub timestamp: u64,
    pub confidence: u8,
}

const ORACLE_CONTRACT: &str = "oracle.near";
const TGAS: u64 = 1_000_000_000_000;

#[near_bindgen]
impl MyContract {
    pub fn get_btc_price(&self) -> Promise {
        ext_oracle::ext(ORACLE_CONTRACT.parse().unwrap())
            .with_static_gas(Gas(5 * TGAS))
            .get_price("BTC".to_string())
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .on_price_received()
            )
    }

    #[private]
    pub fn on_price_received(&self, #[callback_result] price: Result<Option<AssetPrice>, PromiseError>) {
        match price {
            Ok(Some(asset_price)) => {
                let price_value = asset_price.price.0 / 100_000_000; // Convert from 8 decimals
                env::log_str(&format!("BTC Price: ${}", price_value));
                // Use the price in your logic
            }
            Ok(None) => env::log_str("Price not available"),
            Err(_) => env::log_str("Error fetching price"),
        }
    }
}
```

### Example: DeFi Lending Protocol

```rust
#[near_bindgen]
impl LendingProtocol {
    pub fn calculate_collateral_value(&self, token: String, amount: U128) -> Promise {
        ext_oracle::ext(ORACLE_CONTRACT.parse().unwrap())
            .with_static_gas(Gas(5 * TGAS))
            .get_price(token)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(10 * TGAS))
                    .on_collateral_calculated(amount)
            )
    }

    #[private]
    pub fn on_collateral_calculated(
        &mut self,
        amount: U128,
        #[callback_result] price: Result<Option<AssetPrice>, PromiseError>
    ) {
        if let Ok(Some(asset_price)) = price {
            let value = (amount.0 * asset_price.price.0) / 100_000_000;
            // Use value for lending calculations
        }
    }
}
```

## JavaScript/TypeScript Integration

### Installation

```bash
npm install near-api-js
```

### Basic Price Query

```typescript
import * as nearAPI from 'near-api-js'

const { connect, keyStores } = nearAPI

async function getPrice(symbol: string) {
  const near = await connect({
    networkId: 'mainnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.mainnet.near.org',
    walletUrl: 'https://wallet.mainnet.near.org',
  })

  const account = await near.account('oracle.near')
  
  const result = await account.viewFunction({
    contractId: 'oracle.near',
    methodName: 'get_price',
    args: { symbol },
  })

  return result
}

// Usage
const btcPrice = await getPrice('BTC')
console.log(`BTC Price: $${btcPrice.price / 100000000}`)
console.log(`Confidence: ${btcPrice.confidence}%`)
```

### Batch Price Queries

```typescript
async function getBatchPrices(symbols: string[]) {
  const near = await connect({
    networkId: 'mainnet',
    nodeUrl: 'https://rpc.mainnet.near.org',
  })

  const account = await near.account('oracle.near')
  
  const result = await account.viewFunction({
    contractId: 'oracle.near',
    methodName: 'get_prices',
    args: { symbols },
  })

  return result
}

// Usage
const prices = await getBatchPrices(['BTC', 'ETH', 'NEAR'])
prices.forEach(price => {
  if (price) {
    console.log(`${price.symbol}: $${price.price / 100000000}`)
  }
})
```

### Real-time Price Updates

```typescript
class PriceMonitor {
  private intervalId: NodeJS.Timeout | null = null

  async startMonitoring(symbols: string[], callback: (prices: any[]) => void) {
    this.intervalId = setInterval(async () => {
      const prices = await getBatchPrices(symbols)
      callback(prices)
    }, 30000) // Update every 30 seconds
  }

  stopMonitoring() {
    if (this.intervalId) {
      clearInterval(this.intervalId)
    }
  }
}

// Usage
const monitor = new PriceMonitor()
monitor.startMonitoring(['BTC', 'ETH'], (prices) => {
  console.log('Price update:', prices)
  // Update your UI or trigger actions
})
```

## Python Integration

### Installation

```bash
pip install py-near
```

### Basic Price Query

```python
from py_near.account import Account
from py_near.dapps.core import NEAR
import asyncio

async def get_price(symbol: str):
    near = NEAR('mainnet')
    account = Account(account_id='oracle.near', rpc_addr=near.rpc_addr)
    
    result = await account.view_function(
        'oracle.near',
        'get_price',
        {'symbol': symbol}
    )
    
    return result

# Usage
async def main():
    btc_price = await get_price('BTC')
    price_usd = btc_price['price'] / 100_000_000
    print(f"BTC Price: ${price_usd:.2f}")
    print(f"Confidence: {btc_price['confidence']}%")

asyncio.run(main())
```

### Price Alert Bot

```python
import asyncio
from typing import Dict, Callable

class PriceAlertBot:
    def __init__(self, oracle_contract: str):
        self.oracle_contract = oracle_contract
        self.alerts: Dict[str, float] = {}
        
    def add_alert(self, symbol: str, threshold: float, callback: Callable):
        self.alerts[symbol] = {
            'threshold': threshold,
            'callback': callback
        }
    
    async def monitor(self):
        while True:
            for symbol, alert in self.alerts.items():
                price_data = await get_price(symbol)
                current_price = price_data['price'] / 100_000_000
                
                if current_price >= alert['threshold']:
                    alert['callback'](symbol, current_price)
            
            await asyncio.sleep(60)  # Check every minute

# Usage
bot = PriceAlertBot('oracle.near')
bot.add_alert('BTC', 70000, lambda s, p: print(f"Alert! {s} reached ${p}"))
asyncio.run(bot.monitor())
```

## API Reference

### View Methods

#### `get_price(symbol: String) -> Option<AssetPrice>`

Get the current price for a single asset.

**Parameters:**
- `symbol` (string): Asset symbol (e.g., "BTC", "ETH")

**Returns:**
```json
{
  "symbol": "BTC",
  "price": "6723456000000",
  "timestamp": 1234567890000000000,
  "sources": [...],
  "confidence": 98
}
```

#### `get_prices(symbols: Vec<String>) -> Vec<Option<AssetPrice>>`

Get prices for multiple assets in one call.

**Parameters:**
- `symbols` (array): Array of asset symbols

**Returns:** Array of AssetPrice objects

#### `get_supported_assets() -> Vec<String>`

Get list of all supported assets.

**Returns:** Array of asset symbols

#### `get_active_nodes() -> Vec<OracleNode>`

Get list of active oracle nodes.

**Returns:** Array of OracleNode objects

#### `agent_query(query: String) -> String`

Natural language query interface for AI agents.

**Parameters:**
- `query` (string): Natural language question

**Returns:** Human-readable response string

## Best Practices

### 1. Check Price Freshness

Always verify that the price timestamp is recent:

```typescript
function isPriceFresh(timestamp: number, maxAge: number = 300000): boolean {
  const age = Date.now() - timestamp / 1000000 // Convert from nanoseconds
  return age < maxAge // Max age 5 minutes
}
```

### 2. Validate Confidence Score

Only use prices with sufficient confidence:

```typescript
function isPriceReliable(confidence: number, minConfidence: number = 90): boolean {
  return confidence >= minConfidence
}
```

### 3. Handle Errors Gracefully

```typescript
try {
  const price = await getPrice('BTC')
  if (!price) {
    console.error('Price not available')
    return
  }
  if (!isPriceFresh(price.timestamp)) {
    console.warn('Price is stale')
  }
  // Use price
} catch (error) {
  console.error('Error fetching price:', error)
  // Implement fallback or retry logic
}
```

### 4. Use Batch Queries

For multiple assets, always use batch queries:

```typescript
// Good ✅
const prices = await getBatchPrices(['BTC', 'ETH', 'NEAR'])

// Bad ❌
const btc = await getPrice('BTC')
const eth = await getPrice('ETH')
const near = await getPrice('NEAR')
```

### 5. Implement Caching

Cache prices to reduce RPC calls:

```typescript
class PriceCache {
  private cache: Map<string, { price: any; timestamp: number }> = new Map()
  private ttl: number = 30000 // 30 seconds

  async getPrice(symbol: string): Promise<any> {
    const cached = this.cache.get(symbol)
    if (cached && Date.now() - cached.timestamp < this.ttl) {
      return cached.price
    }

    const price = await getPrice(symbol)
    this.cache.set(symbol, { price, timestamp: Date.now() })
    return price
  }
}
```

## Troubleshooting

### Price Not Available

**Problem:** `get_price()` returns `null`

**Solutions:**
- Check if the asset is supported: `get_supported_assets()`
- Verify oracle nodes are active: `get_active_nodes()`
- Wait a few seconds and retry

### Stale Price Data

**Problem:** Price timestamp is too old

**Solutions:**
- Check node status - nodes may be experiencing delays
- Verify network connectivity
- Check the dashboard for system status

### High Gas Usage

**Problem:** Contract calls consuming too much gas

**Solutions:**
- Use batch queries instead of multiple single queries
- Implement caching to reduce calls
- Use view calls (read-only) instead of change calls

### Rate Limiting

**Problem:** Too many requests

**Solutions:**
- Implement client-side rate limiting
- Use batch queries
- Cache results appropriately

## Support

- GitHub Issues: https://github.com/near-tee-oracle/issues
- Discord: [Community Channel]
- Email: support@near-tee-oracle.io

## Next Steps

- Read the [Node Operator Guide](node-operator-guide.md) to run your own node
- Check out the [User Guide](user-guide.md) for dashboard usage
- Review the [Maintenance Plan](maintenance-plan.md) for system updates

