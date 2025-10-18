# NEAR TEE Price Oracle

A secure, TEE-based Price Oracle for NEAR Protocol that provides real-time cryptocurrency prices with hardware-level security guarantees.

## 🎯 Overview

This project provides a comprehensive price oracle solution for NEAR blockchain using Trusted Execution Environments (TEE) for enhanced security and reliability. It includes:

- **Smart Contract**: NEAR-native oracle contract for price storage and node management
- **Oracle Nodes**: TEE-secured nodes fetching prices from multiple sources
- **Dashboard**: Real-time monitoring interface with alerts
- **Shade Agent Framework**: AI agent compatibility for automated queries
- **1-Year Maintenance**: Committed support and updates

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    NEAR Blockchain                      │
│  ┌───────────────────────────────────────────────┐    │
│  │         Oracle Smart Contract                 │    │
│  │  - Price Storage                              │    │
│  │  - Node Management                            │    │
│  │  - Access Control                             │    │
│  │  - Shade Agent Integration                    │    │
│  └───────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                          ▲
                          │ Price Updates + Attestations
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
   ┌────▼────┐       ┌────▼────┐      ┌────▼────┐
   │ Node 1  │       │ Node 2  │      │ Node 3  │
   │ (TEE)   │       │ (TEE)   │      │ (TEE)   │
   │ US-East │       │ EU-West │      │ AP-South│
   └────┬────┘       └────┬────┘      └────┬────┘
        │                 │                 │
        └─────────────────┼─────────────────┘
                          │
                  Multiple Price APIs
           (CoinGecko, Binance, Kraken, etc.)

                          │
                          ▼
                 ┌─────────────────┐
                 │   Dashboard      │
                 │  - Price Display │
                 │  - Node Monitor  │
                 │  - Alerts        │
                 └─────────────────┘
```

## 🚀 Features

### Core Features
- ✅ Real-time price feeds for top crypto assets (BTC, ETH, NEAR, etc.)
- ✅ TEE-based security (Intel SGX & Phala Cloud support)
- ✅ Multi-source aggregation (5-10 APIs per asset)
- ✅ Global node distribution for reliability
- ✅ Smart contract with admin controls
- ✅ Shade Agent Framework compatibility
- ✅ Real-time monitoring dashboard
- ✅ Automated alerts and notifications

### Security Features
- Hardware attestation verification
- Encrypted price data transmission
- Multi-signature admin controls
- Circuit breaker for anomalous prices
- Rate limiting and DoS protection

## 📁 Project Structure

```
TEE/
├── contracts/              # NEAR smart contracts
│   ├── oracle/            # Main oracle contract
│   └── tests/             # Contract tests
├── oracle-node/           # Oracle node service
│   ├── src/
│   │   ├── price-fetcher/ # Multi-API price aggregation
│   │   ├── tee/           # TEE attestation
│   │   └── publisher/     # Price publishing to NEAR
│   └── config/            # Node configuration
├── dashboard/             # Monitoring dashboard
│   ├── components/        # React components
│   ├── pages/             # Next.js pages
│   └── services/          # API services
├── shade-agent/           # Shade Agent Framework integration
├── docs/                  # Documentation
│   ├── developer-guide.md
│   ├── node-operator-guide.md
│   ├── user-guide.md
│   └── maintenance-plan.md
└── scripts/               # Deployment and utility scripts
```

## 🛠️ Technology Stack

- **Smart Contract**: Rust + NEAR SDK
- **Oracle Nodes**: Rust with TEE support
- **Dashboard**: Next.js + React + TypeScript + Tailwind CSS
- **TEE**: Intel SGX SDK / Phala Cloud
- **Price APIs**: CoinGecko, Binance, Kraken, CoinMarketCap, Cryptocompare

## 🏁 Quick Start

### Prerequisites

- Node.js 18+ and npm/yarn
- Rust 1.70+ and Cargo
- NEAR CLI
- Intel SGX SDK (for local TEE testing)
- Docker (optional, for containerized deployment)

### 1. Deploy Smart Contract

```bash
cd contracts/oracle
./build.sh
near dev-deploy --wasmFile res/oracle.wasm
```

### 2. Run Oracle Node

```bash
cd oracle-node
cp config/example.config.toml config/config.toml
# Edit config with your settings
cargo run --release
```

### 3. Launch Dashboard

```bash
cd dashboard
npm install
npm run dev
# Visit http://localhost:3000
```

## 📊 Supported Assets

Initial support for top 20 crypto assets:
- Bitcoin (BTC)
- Ethereum (ETH)
- NEAR Protocol (NEAR)
- BNB, XRP, ADA, SOL, DOT, MATIC, AVAX, etc.

More assets can be added through governance.

## 🔐 Security

### TEE Attestation
All oracle nodes run in Trusted Execution Environments and provide cryptographic attestation:
- **Intel SGX**: Remote attestation via Intel Attestation Service (IAS)
- **Phala Cloud**: Attestation via Phala Network

### Price Validation
- Median aggregation from multiple sources
- Outlier detection and removal
- Minimum source threshold (5 APIs)
- Maximum deviation limits

## 📖 Documentation

- [Developer Guide](docs/developer-guide.md) - Integration and API reference
- [Node Operator Guide](docs/node-operator-guide.md) - Running oracle nodes
- [User Guide](docs/user-guide.md) - Using the dashboard
- [Maintenance Plan](docs/maintenance-plan.md) - 1-year support details
- [API Documentation](docs/api-reference.md) - Contract methods and endpoints

## 🛡️ Maintenance & Support

This project includes **1-year committed maintenance**:

- Monthly security audits and updates
- Bug fixes and performance improvements
- New asset additions (upon request)
- API provider updates
- Node infrastructure monitoring
- 24/7 incident response

See [Maintenance Plan](docs/maintenance-plan.md) for details.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🌐 Links

- [NEAR Protocol](https://near.org/)
- [Shade Agent Framework](https://github.com/shade-labs/shade-agent)
- [Intel SGX](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/overview.html)
- [Phala Network](https://phala.network/)

## 📞 Contact & Support

- GitHub Issues: [Report bugs or request features]
- Discord: [Community support]
- Email: support@near-tee-oracle.io

---

**Built for NEAR TEE Price Oracle Contest**
