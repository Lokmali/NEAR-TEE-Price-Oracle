# NEAR TEE Oracle - Architecture

## System Overview

The NEAR TEE Oracle is a decentralized price oracle system that provides secure, reliable cryptocurrency price data to NEAR Protocol applications using Trusted Execution Environments (TEE).

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                         External Data Sources                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │ Binance  │  │  Kraken  │  │ Coinbase │  │CoinGecko │  ... (10+) │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘           │
└─────────────────────────────────────────────────────────────────────┘
                              │ │ │ │
                              ▼ ▼ ▼ ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Oracle Node Layer                            │
│                                                                       │
│  ┌────────────────────────────────────────────────────────┐         │
│  │            Oracle Node (TEE Protected)                 │         │
│  │  ┌──────────────────────────────────────────────────┐ │         │
│  │  │         Trusted Execution Environment             │ │         │
│  │  │  ┌────────────┐  ┌───────────┐  ┌─────────────┐ │ │         │
│  │  │  │Price       │  │Validation │  │Attestation  │ │ │         │
│  │  │  │Aggregation │→ │& Signing  │→ │Generation   │ │ │         │
│  │  │  └────────────┘  └───────────┘  └─────────────┘ │ │         │
│  │  └──────────────────────────────────────────────────┘ │         │
│  └────────────────────────────────────────────────────────┘         │
│                                                                       │
│  Node 1 (US-East)    Node 2 (EU-West)    Node 3 (AP-South)         │
└─────────────────────────────────────────────────────────────────────┘
                              │ │ │
                    Price Updates + Attestations
                              ▼ ▼ ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        NEAR Blockchain                               │
│                                                                       │
│  ┌────────────────────────────────────────────────────────┐         │
│  │              Oracle Smart Contract                     │         │
│  │                                                         │         │
│  │  ┌──────────────┐  ┌───────────────┐  ┌────────────┐ │         │
│  │  │Price Storage │  │Node Management│  │Access      │ │         │
│  │  │& Validation  │  │& Attestation  │  │Control     │ │         │
│  │  └──────────────┘  └───────────────┘  └────────────┘ │         │
│  │                                                         │         │
│  │  ┌──────────────────────────────────────────────────┐ │         │
│  │  │        Shade Agent Framework Interface           │ │         │
│  │  │  (Natural Language Query Support for AI)         │ │         │
│  │  └──────────────────────────────────────────────────┘ │         │
│  └────────────────────────────────────────────────────────┘         │
└─────────────────────────────────────────────────────────────────────┘
                              │ │ │
                    ┌─────────┘ │ └─────────┐
                    ▼            ▼           ▼
        ┌───────────────┐  ┌─────────┐  ┌──────────────┐
        │   Dashboard   │  │DeFi Apps│  │ AI Agents    │
        │  (Monitoring) │  │         │  │(Shade Agent) │
        └───────────────┘  └─────────┘  └──────────────┘
                    
                         End Users & Applications
```

## Component Architecture

### 1. Oracle Smart Contract

**Location**: `contracts/oracle/`

**Responsibilities**:
- Store and serve price data
- Manage authorized oracle nodes
- Validate TEE attestations
- Enforce access control
- Provide AI agent interface

**Key Features**:
- Written in Rust using NEAR SDK
- Gas-optimized for efficiency
- Admin controls for governance
- Price validation with outlier detection
- Median aggregation algorithm
- Circuit breaker for anomalous data

**Storage Structure**:
```rust
pub struct Oracle {
    owner: AccountId,
    paused: bool,
    nodes: UnorderedMap<AccountId, OracleNode>,
    active_nodes: UnorderedSet<AccountId>,
    prices: UnorderedMap<String, AssetPrice>,
    supported_assets: UnorderedSet<String>,
    min_sources: u8,
    admins: UnorderedSet<AccountId>,
}
```

### 2. Oracle Nodes

**Location**: `oracle-node/`

**Responsibilities**:
- Fetch prices from multiple APIs
- Aggregate and validate data
- Generate TEE attestations
- Submit prices to blockchain
- Maintain high availability

**Architecture**:
```
┌─────────────────────────────────────┐
│         Oracle Node Service          │
├─────────────────────────────────────┤
│                                      │
│  ┌────────────────────────────────┐ │
│  │      Price Fetcher Module      │ │
│  │  - Multi-API client            │ │
│  │  - Rate limiting               │ │
│  │  - Error handling              │ │
│  │  - Timeout management          │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │    Validation & Aggregation    │ │
│  │  - Median calculation          │ │
│  │  - Outlier detection           │ │
│  │  - Confidence scoring          │ │
│  │  - Source validation           │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │      TEE Attestation Module    │ │
│  │  - SGX attestation             │ │
│  │  - Phala attestation           │ │
│  │  - Signature generation        │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │      NEAR Publisher Module     │ │
│  │  - Transaction creation        │ │
│  │  - Gas optimization            │ │
│  │  - Retry logic                 │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**Technology Stack**:
- Language: Rust
- Async Runtime: Tokio
- HTTP Client: Reqwest
- TEE: Intel SGX SDK / Phala SDK
- NEAR Integration: near-jsonrpc-client

### 3. Monitoring Dashboard

**Location**: `dashboard/`

**Responsibilities**:
- Display real-time prices
- Show node status
- Present system health metrics
- Alert on issues
- Provide price history

**Architecture**:
```
┌─────────────────────────────────────┐
│        Dashboard Frontend            │
├─────────────────────────────────────┤
│                                      │
│  ┌────────────────────────────────┐ │
│  │         React Components        │ │
│  │  - PriceGrid                   │ │
│  │  - NodeStatus                  │ │
│  │  - AlertsPanel                 │ │
│  │  - StatsOverview               │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │      Custom Hooks & State      │ │
│  │  - useOracleData               │ │
│  │  - Real-time updates           │ │
│  │  - Error handling              │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │      NEAR API Integration      │ │
│  │  - RPC queries                 │ │
│  │  - Contract view calls         │ │
│  │  - Polling mechanism           │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**Technology Stack**:
- Framework: Next.js 14
- UI: React 18 + TypeScript
- Styling: Tailwind CSS
- Charts: Recharts
- Icons: Lucide React
- NEAR: near-api-js

### 4. Shade Agent Framework Integration

**Location**: `shade-agent/`

**Responsibilities**:
- Provide natural language interface
- Support AI agent queries
- Rate limiting
- Response formatting

**Architecture**:
```
┌─────────────────────────────────────┐
│         Shade Agent Module           │
├─────────────────────────────────────┤
│                                      │
│  ┌────────────────────────────────┐ │
│  │    Natural Language Parser     │ │
│  │  - Query understanding         │ │
│  │  - Asset extraction            │ │
│  │  - Intent classification       │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │       Oracle Query Engine      │ │
│  │  - Contract interaction        │ │
│  │  - Batch optimization          │ │
│  │  - Response formatting         │ │
│  └────────────────────────────────┘ │
│               │                      │
│               ▼                      │
│  ┌────────────────────────────────┐ │
│  │      Rate Limiter & Cache      │ │
│  │  - Request throttling          │ │
│  │  - Response caching            │ │
│  │  - Quota management            │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

## Data Flow

### Price Update Flow

```
1. Oracle Node (every 60 seconds)
   │
   ├─► Fetch prices from 5-10 APIs
   │   ├─► Binance
   │   ├─► Kraken
   │   ├─► Coinbase
   │   └─► Others...
   │
   ├─► Validate & Aggregate
   │   ├─► Remove outliers
   │   ├─► Calculate median
   │   └─► Generate confidence score
   │
   ├─► Generate TEE Attestation
   │   ├─► Create attestation
   │   └─► Sign data
   │
   └─► Submit to NEAR
       ├─► Create transaction
       ├─► Pay gas fee
       └─► Wait for confirmation

2. Smart Contract
   │
   ├─► Verify node authorization
   ├─► Validate attestation
   ├─► Check data integrity
   ├─► Update price storage
   └─► Emit event

3. Dashboard / Applications
   │
   ├─► Poll for updates
   ├─► Display new data
   └─► Trigger alerts if needed
```

### Query Flow

```
1. User / Application
   │
   └─► Query oracle contract
       ├─► get_price("BTC")
       └─► get_prices(["BTC", "ETH", "NEAR"])

2. Smart Contract
   │
   ├─► Lookup price data
   ├─► Verify freshness
   └─► Return AssetPrice object

3. Application
   │
   ├─► Parse price data
   ├─► Check confidence
   └─► Use in business logic
```

## Security Architecture

### TEE (Trusted Execution Environment)

**Intel SGX Implementation**:
```
┌─────────────────────────────────────┐
│         Untrusted Environment        │
│                                      │
│  ┌────────────────────────────────┐ │
│  │     Trusted Environment (SGX)  │ │
│  │                                │ │
│  │  ┌──────────────────────────┐ │ │
│  │  │   Enclave Code           │ │ │
│  │  │  - Price fetching        │ │ │
│  │  │  - Validation logic      │ │ │
│  │  │  - Signing keys          │ │ │
│  │  └──────────────────────────┘ │ │
│  │                                │ │
│  │  Features:                     │ │
│  │  - Memory encryption           │ │
│  │  - Remote attestation          │ │
│  │  - Sealed storage              │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### Access Control

```
┌─────────────────────────────────────┐
│        Permission Levels             │
├─────────────────────────────────────┤
│                                      │
│  Owner (Single Account)              │
│  └─► Full control                   │
│      ├─► Pause/Resume               │
│      ├─► Add/Remove admins          │
│      └─► Emergency actions          │
│                                      │
│  Admins (Multiple Accounts)          │
│  └─► Management functions           │
│      ├─► Register nodes             │
│      ├─► Add/Remove assets          │
│      └─► Configure parameters       │
│                                      │
│  Oracle Nodes (Registered)           │
│  └─► Price submission               │
│      └─► submit_price()             │
│                                      │
│  Public (Anyone)                     │
│  └─► Read-only access               │
│      ├─► get_price()                │
│      ├─► get_prices()               │
│      └─► agent_query()              │
└─────────────────────────────────────┘
```

## Scalability

### Horizontal Scaling

- **Nodes**: Add more nodes for redundancy
- **Geographic Distribution**: Nodes in multiple regions
- **Load Balancing**: Multiple nodes share workload

### Vertical Scaling

- **Batch Processing**: Update multiple assets per transaction
- **Gas Optimization**: Efficient contract code
- **Caching**: Client-side and node-side caching

### Performance Metrics

- **Throughput**: 100+ price updates per minute
- **Latency**: < 500ms for read queries
- **Uptime**: > 99.5% target
- **Price Freshness**: < 60 seconds

## Monitoring & Observability

### Metrics Collected

1. **Node Metrics**
   - Uptime
   - Update frequency
   - API response times
   - Error rates

2. **Contract Metrics**
   - Price freshness
   - Confidence scores
   - Gas usage
   - Storage usage

3. **System Metrics**
   - Network latency
   - Database performance
   - Cache hit rates

### Alerting

- Node offline > 5 minutes
- Price data stale > 5 minutes
- Low confidence scores
- High error rates
- Low gas balance

## Disaster Recovery

### Backup Strategy

- **Configuration**: Git repository
- **Keys**: Secure key management service
- **State**: On-chain (NEAR blockchain)
- **Logs**: Centralized logging service

### Recovery Procedures

1. **Node Failure**: Automatic failover to other nodes
2. **Contract Issue**: Pause contract, investigate, fix, resume
3. **Data Corruption**: Rollback to last known good state
4. **Complete Outage**: Deploy from backups, verify integrity

## Future Enhancements

### Roadmap

**Q1 2024**
- Multi-chain support (Ethereum, BSC)
- Historical data API
- WebSocket price feeds
- Advanced analytics

**Q2 2024**
- Decentralized governance
- Staking mechanism for nodes
- Custom asset requests
- Price prediction models

**Q3 2024**
- Cross-chain price verification
- Privacy-preserving queries
- Enterprise API tier
- Mobile dashboard app

**Q4 2024**
- DAO treasury management
- Automated node scaling
- ML-based anomaly detection
- Advanced monitoring tools

---

**Last Updated**: 2024
**Version**: 1.0
**Status**: Production Ready

