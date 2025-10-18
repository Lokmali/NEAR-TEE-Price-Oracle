# Quick Start Guide

Get started with NEAR TEE Oracle in 5 minutes!

## What You'll Build

By the end of this guide, you'll have:
- ✅ Oracle smart contract deployed on NEAR testnet
- ✅ Oracle node running locally
- ✅ Dashboard running and displaying live data
- ✅ Understanding of how to integrate the oracle

## Prerequisites

- **Operating System**: Ubuntu 20.04+ or macOS
- **Node.js**: Version 18 or higher
- **Rust**: Version 1.70 or higher
- **NEAR CLI**: Latest version
- **Git**: For cloning the repository

### Install Prerequisites

**On Ubuntu/Debian:**
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install NEAR CLI
npm install -g near-cli

# Install build tools
sudo apt install -y build-essential pkg-config libssl-dev git
```

**On macOS:**
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install node rust git

# Install NEAR CLI
npm install -g near-cli
```

## Step 1: Clone the Repository

```bash
git clone https://github.com/near-tee-oracle/near-tee-oracle.git
cd near-tee-oracle
```

## Step 2: Deploy the Smart Contract

```bash
# Make deployment script executable
chmod +x scripts/deploy-contract.sh

# Deploy to testnet (this will also create an account if needed)
./scripts/deploy-contract.sh testnet oracle-test-$(date +%s).testnet 5

# Expected output:
# Contract deployed to: oracle-test-1234567890.testnet
# Supported assets: BTC, ETH, NEAR, BNB, SOL, ADA, DOT, MATIC, AVAX, XRP
```

**Note the contract ID** - you'll need it for the next steps!

## Step 3: Configure and Run Oracle Node

```bash
# Navigate to oracle node directory
cd oracle-node

# Copy example config
cp config/example.config.toml config/config.toml

# Edit configuration
nano config/config.toml
```

**Update these values in config.toml:**

```toml
node_id = "my-test-node"
region = "local"
endpoint = "http://localhost:8080"

# Assets to track (start with a few for testing)
assets = ["BTC", "ETH", "NEAR"]

[near]
network = "testnet"
rpc_url = "https://rpc.testnet.near.org"
contract_id = "oracle-test-1234567890.testnet"  # YOUR CONTRACT ID HERE
signer_id = "your-account.testnet"  # YOUR NEAR ACCOUNT
private_key = "ed25519:YOUR_PRIVATE_KEY"  # YOUR PRIVATE KEY

[tee]
tee_type = "sgx"
enable_attestation = false  # Disabled for local testing

[price_sources]
binance_enabled = true
kraken_enabled = true
coinbase_enabled = true
huobi_enabled = true
kucoin_enabled = true
```

**Get your NEAR credentials:**

```bash
# Create a testnet account if you don't have one
near create-account your-account.testnet --useFaucet

# View your private key
cat ~/.near-credentials/testnet/your-account.testnet.json

# Copy the "private_key" value to your config
```

**Register your node with the contract:**

```bash
# From the root directory
near call oracle-test-1234567890.testnet register_node \
  '{"node_id": "my-test-node.testnet", "tee_type": "sgx", "attestation": "test", "region": "local", "endpoint": "http://localhost:8080"}' \
  --accountId oracle-test-1234567890.testnet \
  --gas 50000000000000
```

**Build and run the node:**

```bash
# Build (first time will take a few minutes)
cargo build --release

# Run
RUST_LOG=info cargo run --release

# You should see:
# INFO oracle_node: Starting NEAR TEE Oracle Node...
# INFO oracle_node: Configuration loaded successfully
# INFO oracle_node: Connected to NEAR network: testnet
# INFO oracle_node: Oracle node is running
```

Keep this terminal open and running!

## Step 4: Run the Dashboard

Open a **new terminal**:

```bash
cd dashboard

# Install dependencies (first time only)
npm install

# Start development server
npm run dev

# Dashboard will be available at:
# http://localhost:3000
```

Open your browser and visit **http://localhost:3000**

You should see:
- Live prices for BTC, ETH, NEAR
- Your oracle node status
- Real-time updates every 60 seconds

## Step 5: Test the Oracle

### Test 1: Query Price from CLI

```bash
# Query BTC price
near view oracle-test-1234567890.testnet get_price '{"symbol": "BTC"}'

# Expected output:
# {
#   symbol: 'BTC',
#   price: '6723456000000',
#   timestamp: 1234567890000000000,
#   confidence: 98,
#   sources: [...]
# }
```

### Test 2: Query Multiple Prices

```bash
near view oracle-test-1234567890.testnet get_prices '{"symbols": ["BTC", "ETH", "NEAR"]}'
```

### Test 3: Use Shade Agent (AI Query)

```bash
cd shade-agent

# Install dependencies
npm install

# Edit config to use your contract
nano agent-config.json

# Update contract_id to your deployed contract

# Run interactive CLI
npm run cli

# Try queries like:
# "What is the price of Bitcoin?"
# "How much is Ethereum?"
```

## Step 6: Integrate into Your App

### Example: Query Oracle in Your NEAR Contract

```rust
use near_sdk::ext_contract;

#[ext_contract(ext_oracle)]
pub trait OracleContract {
    fn get_price(&self, symbol: String) -> Option<AssetPrice>;
}

#[near_bindgen]
impl MyContract {
    pub fn get_btc_price(&self) -> Promise {
        ext_oracle::ext("oracle-test-1234567890.testnet".parse().unwrap())
            .with_static_gas(Gas(5_000_000_000_000))
            .get_price("BTC".to_string())
    }
}
```

### Example: Query Oracle from JavaScript

```javascript
import * as nearAPI from 'near-api-js'

async function getPrice(symbol) {
  const near = await nearAPI.connect({
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org'
  })
  
  const account = await near.account('oracle-test-1234567890.testnet')
  
  const price = await account.viewFunction({
    contractId: 'oracle-test-1234567890.testnet',
    methodName: 'get_price',
    args: { symbol }
  })
  
  return price
}

// Use it
const btcPrice = await getPrice('BTC')
console.log(`BTC: $${btcPrice.price / 100000000}`)
```

## Next Steps

### Learn More

- 📖 [Developer Guide](docs/developer-guide.md) - Detailed integration guide
- 🖥️ [Node Operator Guide](docs/node-operator-guide.md) - Run a production node
- 📊 [User Guide](docs/user-guide.md) - Using the dashboard
- 🔧 [API Reference](docs/api-reference.md) - Complete API docs

### Deploy to Production

Ready for mainnet? Follow these steps:

1. **Deploy contract to mainnet:**
   ```bash
   ./scripts/deploy-contract.sh mainnet oracle.near 5
   ```

2. **Configure production node:**
   - Use production NEAR account with sufficient balance
   - Enable TEE attestation
   - Use production API keys
   - Set up monitoring and alerts

3. **Deploy dashboard:**
   ```bash
   ./scripts/deploy-dashboard.sh vercel production
   ```

### Get Support

- **Discord**: [Join our community]
- **GitHub**: [Open an issue](https://github.com/near-tee-oracle/issues)
- **Email**: support@near-tee-oracle.io

## Troubleshooting

### Node Won't Start

**Error: "Failed to parse private key"**
- Check that your private key in config.toml is correct
- Ensure it includes the "ed25519:" prefix

**Error: "Node not registered"**
- Register your node with the contract using `register_node`
- Ensure you're using the correct contract ID

### No Prices Showing

**Issue: Dashboard shows "Loading" forever**
- Check that your node is running (look for logs)
- Verify node can connect to NEAR RPC
- Check that node has registered with contract

**Issue: "Insufficient price sources"**
- Some APIs may be rate limiting - wait a minute and retry
- Enable more price sources in config.toml
- Add API keys for premium sources

### Contract Errors

**Error: "Unauthorized"**
- Ensure you're calling from the correct account
- Check admin permissions

**Error: "Asset not supported"**
- Add the asset using `add_asset` method
- Check supported assets with `get_supported_assets`

## Common Commands Reference

```bash
# Contract deployment
./scripts/deploy-contract.sh [network] [account-id] [min-sources]

# Node operations
cd oracle-node
cargo run --release                    # Run node
RUST_LOG=debug cargo run --release    # Run with debug logs

# Dashboard
cd dashboard
npm run dev        # Development
npm run build      # Production build
npm run start      # Production server

# Monitoring
./scripts/monitor.sh [contract-id] [network]

# View contract state
near state oracle.testnet --networkId testnet

# Query prices
near view oracle.testnet get_price '{"symbol": "BTC"}'

# Get node info
near view oracle.testnet get_node '{"node_id": "node.testnet"}'
```

## System Requirements

### Minimum (Development/Testing)
- 2 CPU cores
- 4 GB RAM
- 20 GB storage
- 10 Mbps network

### Recommended (Production)
- 4+ CPU cores (Intel with SGX for TEE)
- 16 GB RAM
- 100 GB SSD storage
- 100+ Mbps network with static IP

---

**Congratulations!** 🎉 You now have a fully functional NEAR TEE Oracle system running!

Next, explore the documentation to learn about advanced features, production deployment, and integration patterns.

