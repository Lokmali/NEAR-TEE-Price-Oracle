# Node Operator Guide

This guide will help you set up and operate a NEAR TEE Oracle node.

## Table of Contents

1. [Overview](#overview)
2. [Requirements](#requirements)
3. [Hardware Setup](#hardware-setup)
4. [TEE Setup](#tee-setup)
5. [Node Installation](#node-installation)
6. [Configuration](#configuration)
7. [Running the Node](#running-the-node)
8. [Monitoring](#monitoring)
9. [Maintenance](#maintenance)
10. [Troubleshooting](#troubleshooting)

## Overview

Oracle nodes fetch cryptocurrency prices from multiple sources and submit them to the NEAR blockchain. Nodes run in Trusted Execution Environments (TEE) for enhanced security.

### Responsibilities

- Fetch prices from 5-10 different APIs
- Aggregate and validate price data
- Submit prices to the smart contract
- Maintain high uptime (>99%)
- Generate TEE attestations

### Rewards

Node operators may receive compensation for:
- Uptime and reliability
- Data accuracy
- Response time
- Geographic diversity

## Requirements

### Hardware Requirements

**Minimum:**
- CPU: Intel 8th Gen or AMD Zen 2 with SGX support
- RAM: 8 GB
- Storage: 50 GB SSD
- Network: 100 Mbps, static IP recommended

**Recommended:**
- CPU: Intel 10th Gen or newer with SGX
- RAM: 16 GB
- Storage: 100 GB NVMe SSD
- Network: 1 Gbps, static IP

### Software Requirements

- Ubuntu 20.04 LTS or 22.04 LTS
- Docker 24.0+ (optional)
- Rust 1.70+ (for building from source)
- Intel SGX drivers (for SGX mode)
- NEAR CLI

### TEE Requirements

Choose one of the following:

**Option 1: Intel SGX**
- Intel CPU with SGX support
- SGX enabled in BIOS
- Intel SGX driver and SDK installed

**Option 2: Phala Cloud**
- Phala Network account
- TEE Worker registered with Phala

## Hardware Setup

### Check SGX Support

```bash
# Install cpuid
sudo apt install cpuid

# Check for SGX support
cpuid | grep SGX

# You should see:
# SGX: Software Guard Extensions supported = true
```

### Enable SGX in BIOS

1. Reboot and enter BIOS/UEFI settings
2. Navigate to Security or Advanced settings
3. Enable Intel SGX (set to "Enabled" or "Software Controlled")
4. Save and reboot

### Verify SGX Status

```bash
# Install SGX test tool
git clone https://github.com/ayeks/SGX-hardware.git
cd SGX-hardware
make
./test-sgx

# Expected output: "SGX is ready to use"
```

## TEE Setup

### Option 1: Intel SGX Setup

#### Install SGX Driver

```bash
# Download SGX driver
wget https://download.01.org/intel-sgx/latest/linux-latest/distro/ubuntu22.04-server/sgx_linux_x64_driver_2.11.0.bin

# Make executable and install
chmod +x sgx_linux_x64_driver_*.bin
sudo ./sgx_linux_x64_driver_*.bin

# Verify installation
ls /dev/isgx  # Should exist
```

#### Install SGX SDK

```bash
# Add Intel SGX repository
echo 'deb [arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu jammy main' | sudo tee /etc/apt/sources.list.d/intel-sgx.list
wget -qO - https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | sudo apt-key add -

# Install SGX packages
sudo apt update
sudo apt install -y \
    libsgx-enclave-common \
    libsgx-urts \
    libsgx-launch \
    libsgx-epid \
    libsgx-quote-ex

# Verify installation
sgx-detect
```

### Option 2: Phala Cloud Setup

```bash
# Install Phala tools
curl -fsSL https://github.com/Phala-Network/phala-blockchain/releases/latest/download/phala-tools.sh | bash

# Initialize Phala worker
phala-tools init

# Register with Phala Network
phala-tools register --endpoint wss://poc5.phala.network/ws
```

## Node Installation

### From Binary (Recommended)

```bash
# Download latest release
wget https://github.com/near-tee-oracle/oracle-node/releases/latest/download/oracle-node-linux-amd64

# Make executable
chmod +x oracle-node-linux-amd64
sudo mv oracle-node-linux-amd64 /usr/local/bin/oracle-node

# Verify installation
oracle-node --version
```

### From Source

```bash
# Clone repository
git clone https://github.com/near-tee-oracle/oracle-node.git
cd oracle-node

# Build
cargo build --release

# Install
sudo cp target/release/oracle-node /usr/local/bin/
```

### Using Docker

```bash
# Pull image
docker pull nearteeoracle/oracle-node:latest

# Or build from source
docker build -t oracle-node .
```

## Configuration

### Create Configuration File

```bash
# Create config directory
sudo mkdir -p /etc/oracle-node
sudo cp config/example.config.toml /etc/oracle-node/config.toml
```

### Edit Configuration

```toml
# /etc/oracle-node/config.toml

node_id = "your-node-name"
region = "us-east-1"  # Your geographic region
endpoint = "https://your-node-domain.com"  # Your public endpoint
update_interval = 60  # Update frequency in seconds

# Assets to track
assets = ["BTC", "ETH", "NEAR", "BNB", "SOL", "ADA", "DOT", "MATIC", "AVAX", "XRP"]

[near]
network = "mainnet"  # or "testnet"
rpc_url = "https://rpc.mainnet.near.org"
contract_id = "oracle.near"
signer_id = "your-node.near"
private_key = "ed25519:YOUR_PRIVATE_KEY"

[tee]
tee_type = "sgx"  # or "phala"
enable_attestation = true
attestation_url = "https://api.trustedservices.intel.com/sgx/dev/attestation/v4"

[price_sources]
# Enable/disable sources
binance_enabled = true
kraken_enabled = true
coinbase_enabled = true
huobi_enabled = true
kucoin_enabled = true
bitfinex_enabled = true
gemini_enabled = true

# API keys (optional but recommended for higher rate limits)
coingecko_api_key = "YOUR_COINGECKO_KEY"
coinmarketcap_api_key = "YOUR_CMC_KEY"
cryptocompare_api_key = "YOUR_CRYPTOCOMPARE_KEY"
```

### Generate NEAR Keys

```bash
# Create NEAR account for your node
near create-account your-node.near --useFaucet

# Or use an existing account
near generate-key your-node.near

# Keys will be stored in ~/.near-credentials/
```

### Register Node with Oracle Contract

```bash
# Register your node (requires admin approval)
near call oracle.near register_node \
  '{"node_id": "your-node.near", "tee_type": "sgx", "attestation": "BASE64_ATTESTATION", "region": "us-east-1", "endpoint": "https://your-node.com"}' \
  --accountId admin.near \
  --gas 50000000000000
```

## Running the Node

### Manual Start

```bash
# Start node
oracle-node

# With custom config
oracle-node --config /path/to/config.toml

# With logging
RUST_LOG=info oracle-node
```

### As a Systemd Service

Create service file:

```bash
sudo nano /etc/systemd/system/oracle-node.service
```

```ini
[Unit]
Description=NEAR TEE Oracle Node
After=network.target

[Service]
Type=simple
User=oracle
WorkingDirectory=/opt/oracle-node
ExecStart=/usr/local/bin/oracle-node --config /etc/oracle-node/config.toml
Restart=always
RestartSec=10
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable oracle-node
sudo systemctl start oracle-node

# Check status
sudo systemctl status oracle-node

# View logs
sudo journalctl -u oracle-node -f
```

### Using Docker

```bash
# Run with docker
docker run -d \
  --name oracle-node \
  -v /etc/oracle-node:/config \
  -v /var/log/oracle-node:/logs \
  --restart unless-stopped \
  nearteeoracle/oracle-node:latest \
  --config /config/config.toml

# View logs
docker logs -f oracle-node
```

### Using Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  oracle-node:
    image: nearteeoracle/oracle-node:latest
    container_name: oracle-node
    restart: unless-stopped
    volumes:
      - ./config:/config
      - ./logs:/logs
    environment:
      - RUST_LOG=info
    command: --config /config/config.toml
```

```bash
docker-compose up -d
```

## Monitoring

### Node Health Check

```bash
# Check if node is running
systemctl status oracle-node

# Check recent price updates
near view oracle.near get_node '{"node_id": "your-node.near"}'
```

### Logs

```bash
# Systemd logs
sudo journalctl -u oracle-node -n 100

# Follow logs
sudo journalctl -u oracle-node -f

# Docker logs
docker logs oracle-node --tail 100 -f
```

### Metrics

Monitor these key metrics:

- **Uptime**: Should be >99%
- **Update frequency**: Every 60 seconds
- **Price sources**: At least 5-10 sources per asset
- **Gas usage**: Monitor NEAR token balance
- **Network latency**: <500ms to RPC

### Dashboard

Visit the oracle dashboard to monitor your node:
- https://dashboard.near-tee-oracle.io

Your node should appear in the "Oracle Nodes" section with:
- Green status indicator
- Recent update timestamp
- Total updates count

## Maintenance

### Update Node Software

```bash
# Stop node
sudo systemctl stop oracle-node

# Download latest version
wget https://github.com/near-tee-oracle/oracle-node/releases/latest/download/oracle-node-linux-amd64
sudo mv oracle-node-linux-amd64 /usr/local/bin/oracle-node
sudo chmod +x /usr/local/bin/oracle-node

# Start node
sudo systemctl start oracle-node
```

### Renew TEE Attestation

Attestations expire periodically and must be renewed:

```bash
# Generate new attestation
oracle-node generate-attestation

# Update on contract
near call oracle.near update_node_attestation \
  '{"attestation": "NEW_ATTESTATION"}' \
  --accountId your-node.near
```

### Rotate Keys

```bash
# Generate new key
near generate-key your-node.near --seedPhrase "your seed phrase"

# Update config with new key
sudo nano /etc/oracle-node/config.toml

# Restart node
sudo systemctl restart oracle-node
```

### Backup Configuration

```bash
# Backup config and keys
sudo tar czf oracle-node-backup.tar.gz \
  /etc/oracle-node/ \
  ~/.near-credentials/

# Store securely off-server
```

## Troubleshooting

### Node Not Starting

**Check logs:**
```bash
sudo journalctl -u oracle-node -n 50
```

**Common issues:**
- Invalid private key → Regenerate NEAR keys
- Missing config → Verify config.toml exists
- SGX not available → Check SGX setup

### Price Updates Failing

**Check:**
```bash
# Test API connectivity
curl https://api.coingecko.com/api/v3/ping

# Test NEAR RPC
curl https://rpc.mainnet.near.org/status
```

**Common issues:**
- API rate limits → Add API keys to config
- Network connectivity → Check firewall rules
- Insufficient balance → Add NEAR tokens for gas

### High Gas Usage

**Optimize:**
- Increase update_interval to reduce frequency
- Remove redundant assets
- Batch updates if possible

### TEE Attestation Fails

**For SGX:**
```bash
# Verify SGX is working
sgx-detect

# Reinstall SGX driver if needed
```

**For Phala:**
```bash
# Check Phala worker status
phala-tools status

# Restart worker
phala-tools restart
```

## Support

- **Documentation**: https://docs.near-tee-oracle.io
- **GitHub**: https://github.com/near-tee-oracle
- **Discord**: [Node Operators Channel]
- **Email**: operators@near-tee-oracle.io

## Best Practices

1. **Security**
   - Keep system updated
   - Use firewall (UFW)
   - Limit SSH access
   - Secure private keys

2. **Reliability**
   - Monitor uptime
   - Set up alerts
   - Have backup power
   - Use monitoring tools

3. **Performance**
   - Use SSD storage
   - Adequate RAM
   - Fast network connection
   - Geographic diversity

4. **Compliance**
   - Follow operator guidelines
   - Maintain attestations
   - Report issues promptly
   - Participate in governance

---

Thank you for operating a NEAR TEE Oracle node! Your contribution helps secure the oracle network and provides reliable price data to the NEAR ecosystem.

