#!/bin/bash

# NEAR TEE Oracle - Contract Deployment Script
# This script deploys the oracle smart contract to NEAR

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NETWORK=${1:-testnet}
ACCOUNT_ID=${2:-oracle.$NETWORK}
MIN_SOURCES=${3:-5}

echo -e "${GREEN}NEAR TEE Oracle - Contract Deployment${NC}"
echo "========================================"
echo "Network: $NETWORK"
echo "Account: $ACCOUNT_ID"
echo "Min Sources: $MIN_SOURCES"
echo ""

# Check if NEAR CLI is installed
if ! command -v near &> /dev/null; then
    echo -e "${RED}Error: NEAR CLI not found${NC}"
    echo "Install with: npm install -g near-cli"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found${NC}"
    echo "Install from: https://rustup.rs/"
    exit 1
fi

# Build the contract
echo -e "${YELLOW}Building contract...${NC}"
cd contracts/oracle
./build.sh

if [ ! -f "res/oracle.wasm" ]; then
    echo -e "${RED}Error: Contract build failed${NC}"
    exit 1
fi

echo -e "${GREEN}Contract built successfully${NC}"
echo ""

# Check if account exists
echo -e "${YELLOW}Checking account status...${NC}"
if ! near state $ACCOUNT_ID --networkId $NETWORK &> /dev/null; then
    echo -e "${YELLOW}Account $ACCOUNT_ID does not exist${NC}"
    
    if [ "$NETWORK" == "testnet" ]; then
        echo "Creating testnet account..."
        near create-account $ACCOUNT_ID --useFaucet
    else
        echo -e "${RED}Error: Account does not exist on mainnet${NC}"
        echo "Please create the account first"
        exit 1
    fi
fi

echo -e "${GREEN}Account exists${NC}"
echo ""

# Deploy the contract
echo -e "${YELLOW}Deploying contract...${NC}"
near deploy \
    --accountId $ACCOUNT_ID \
    --wasmFile res/oracle.wasm \
    --networkId $NETWORK

echo -e "${GREEN}Contract deployed successfully${NC}"
echo ""

# Initialize the contract
echo -e "${YELLOW}Initializing contract...${NC}"
near call $ACCOUNT_ID new \
    "{\"owner\": \"$ACCOUNT_ID\", \"min_sources\": $MIN_SOURCES}" \
    --accountId $ACCOUNT_ID \
    --networkId $NETWORK \
    --gas 50000000000000

echo -e "${GREEN}Contract initialized${NC}"
echo ""

# Add default assets
echo -e "${YELLOW}Adding default assets...${NC}"
ASSETS=("BTC" "ETH" "NEAR" "BNB" "SOL" "ADA" "DOT" "MATIC" "AVAX" "XRP")

for asset in "${ASSETS[@]}"; do
    echo "Adding $asset..."
    near call $ACCOUNT_ID add_asset \
        "{\"symbol\": \"$asset\"}" \
        --accountId $ACCOUNT_ID \
        --networkId $NETWORK \
        --gas 5000000000000
done

echo -e "${GREEN}Assets added${NC}"
echo ""

# Display contract info
echo -e "${GREEN}Deployment Complete!${NC}"
echo "===================="
echo "Contract ID: $ACCOUNT_ID"
echo "Network: $NETWORK"
echo ""
echo "Supported Assets:"
near view $ACCOUNT_ID get_supported_assets '{}' --networkId $NETWORK
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Register oracle nodes with register_node"
echo "2. Configure nodes to point to this contract"
echo "3. Start node services"
echo ""
echo "View contract state:"
echo "near state $ACCOUNT_ID --networkId $NETWORK"

