#!/bin/bash

# NEAR TEE Oracle - Node Setup Script
# This script sets up an oracle node on a fresh Ubuntu server

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}NEAR TEE Oracle - Node Setup${NC}"
echo "=============================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Please run as root (use sudo)${NC}"
    exit 1
fi

# Update system
echo -e "${YELLOW}Updating system packages...${NC}"
apt-get update
apt-get upgrade -y

# Install dependencies
echo -e "${YELLOW}Installing dependencies...${NC}"
apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    wget \
    cpuid

# Install Rust
echo -e "${YELLOW}Installing Rust...${NC}"
if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install Node.js (for NEAR CLI)
echo -e "${YELLOW}Installing Node.js...${NC}"
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
    apt-get install -y nodejs
fi

# Install NEAR CLI
echo -e "${YELLOW}Installing NEAR CLI...${NC}"
npm install -g near-cli

# Check SGX support
echo -e "${YELLOW}Checking SGX support...${NC}"
if cpuid | grep -q "SGX"; then
    echo -e "${GREEN}SGX is supported${NC}"
    
    # Prompt to install SGX driver
    read -p "Install Intel SGX driver? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Installing SGX driver...${NC}"
        
        # Download SGX driver
        cd /tmp
        wget https://download.01.org/intel-sgx/latest/linux-latest/distro/ubuntu22.04-server/sgx_linux_x64_driver_2.11.0.bin
        chmod +x sgx_linux_x64_driver_*.bin
        ./sgx_linux_x64_driver_*.bin
        
        # Add SGX repository
        echo 'deb [arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu jammy main' | tee /etc/apt/sources.list.d/intel-sgx.list
        wget -qO - https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | apt-key add -
        
        # Install SGX packages
        apt-get update
        apt-get install -y \
            libsgx-enclave-common \
            libsgx-urts \
            libsgx-launch \
            libsgx-epid \
            libsgx-quote-ex
        
        echo -e "${GREEN}SGX driver installed${NC}"
    fi
else
    echo -e "${YELLOW}SGX not detected. TEE attestation will be disabled.${NC}"
fi

# Clone oracle-node repository
echo -e "${YELLOW}Setting up oracle node...${NC}"
cd /opt
if [ -d "oracle-node" ]; then
    echo "oracle-node directory already exists, pulling latest..."
    cd oracle-node
    git pull
else
    git clone https://github.com/near-tee-oracle/oracle-node.git
    cd oracle-node
fi

# Build oracle-node
echo -e "${YELLOW}Building oracle node...${NC}"
cargo build --release

# Install binary
cp target/release/oracle-node /usr/local/bin/
chmod +x /usr/local/bin/oracle-node

# Create config directory
mkdir -p /etc/oracle-node

# Copy example config
cp config/example.config.toml /etc/oracle-node/config.toml

# Create oracle user
if ! id -u oracle &> /dev/null; then
    useradd -r -s /bin/false oracle
fi

# Set permissions
chown -R oracle:oracle /etc/oracle-node
chown -R oracle:oracle /opt/oracle-node

# Create log directory
mkdir -p /var/log/oracle-node
chown oracle:oracle /var/log/oracle-node

# Create systemd service
cat > /etc/systemd/system/oracle-node.service << EOF
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
StandardOutput=append:/var/log/oracle-node/output.log
StandardError=append:/var/log/oracle-node/error.log

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
systemctl daemon-reload

echo -e "${GREEN}Setup complete!${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Edit configuration: nano /etc/oracle-node/config.toml"
echo "   - Set your node_id, region, endpoint"
echo "   - Add your NEAR account credentials"
echo "   - Configure price sources"
echo ""
echo "2. Generate NEAR keys:"
echo "   near generate-key your-node.near"
echo ""
echo "3. Register node with oracle contract:"
echo "   (Get admin approval first)"
echo ""
echo "4. Start the node:"
echo "   systemctl enable oracle-node"
echo "   systemctl start oracle-node"
echo ""
echo "5. Check status:"
echo "   systemctl status oracle-node"
echo "   journalctl -u oracle-node -f"
echo ""
echo "Configuration file: /etc/oracle-node/config.toml"
echo "Logs: /var/log/oracle-node/"
echo ""
echo -e "${GREEN}Happy oracle node operating!${NC}"

