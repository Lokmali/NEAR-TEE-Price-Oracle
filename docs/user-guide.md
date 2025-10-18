# User Guide

Welcome to the NEAR TEE Oracle user guide. This document will help you use the dashboard and understand the oracle system.

## Table of Contents

1. [Introduction](#introduction)
2. [Dashboard Overview](#dashboard-overview)
3. [Reading Prices](#reading-prices)
4. [Understanding Data](#understanding-data)
5. [Node Status](#node-status)
6. [Alerts](#alerts)
7. [FAQ](#faq)

## Introduction

The NEAR TEE Oracle provides real-time cryptocurrency prices secured by Trusted Execution Environments (TEE). The dashboard lets you monitor prices, node status, and system health.

## Dashboard Overview

### Access the Dashboard

Visit: https://dashboard.near-tee-oracle.io

No login required! The dashboard is publicly accessible.

### Main Sections

1. **Stats Overview** - Key metrics at a glance
2. **Live Prices** - Real-time cryptocurrency prices
3. **Oracle Nodes** - Status of oracle nodes
4. **Alerts** - System notifications

## Reading Prices

### Price Cards

Each asset displays:
- **Symbol**: Asset ticker (BTC, ETH, etc.)
- **Current Price**: Latest price in USD
- **24h Change**: Price change percentage
- **Confidence**: Data reliability score (0-100%)
- **Sources**: Number of price sources
- **Last Update**: Time since last update

### Price Color Coding

- **Green**: Price increased in last 24h
- **Red**: Price decreased in last 24h
- **Gray**: No significant change

### Confidence Score

The confidence score indicates data quality:

- **95-100%**: Excellent - All sources agree closely
- **85-94%**: Good - Minor variation between sources
- **70-84%**: Fair - Moderate variation
- **<70%**: Low - Significant variation (use caution)

### Update Frequency

Prices update every **60 seconds**. The "Last Update" field shows how recently the data was refreshed.

## Understanding Data

### How Prices Are Calculated

1. **Fetching**: Each node queries 5-10 different exchanges/APIs
2. **Validation**: Outliers are removed
3. **Aggregation**: Median price is calculated
4. **Verification**: Confidence score is computed
5. **Publishing**: Price is submitted to NEAR blockchain

### Price Sources

We aggregate data from:
- Binance
- Kraken
- Coinbase
- CoinGecko
- CoinMarketCap
- Huobi
- KuCoin
- Bitfinex
- Gemini
- CryptoCompare

### Supported Assets

Current supported cryptocurrencies:
- Bitcoin (BTC)
- Ethereum (ETH)
- NEAR Protocol (NEAR)
- Binance Coin (BNB)
- Solana (SOL)
- Cardano (ADA)
- Polkadot (DOT)
- Polygon (MATIC)
- Avalanche (AVAX)
- Ripple (XRP)

More assets can be added through governance.

## Node Status

### Node Cards

Each oracle node displays:
- **Account ID**: NEAR account running the node
- **Status**: Active/Inactive
- **TEE Type**: SGX or Phala
- **Region**: Geographic location
- **Total Updates**: Lifetime price submissions
- **Last Update**: Time since last submission
- **Endpoint**: Node API endpoint

### Status Indicators

- **Green Dot**: Node is healthy (updated within 2 minutes)
- **Yellow Dot**: Node is slow (updated within 5 minutes)
- **Red Dot**: Node is delayed (>5 minutes since update)
- **Gray Dot**: Node is inactive

### Geographic Distribution

Nodes are distributed globally for reliability:
- North America (US East, US West)
- Europe (EU West, EU Central)
- Asia Pacific (Singapore, Tokyo, Mumbai)

## Alerts

### Alert Types

**Error (Red)**
- Node failure
- Price data unavailable
- System critical issues

**Warning (Yellow)**
- Node delayed updates
- Low confidence scores
- Unusual price movements

**Info (Blue)**
- System updates
- New assets added
- Scheduled maintenance

### Dismissing Alerts

Click the **X** button on any alert to dismiss it. Dismissed alerts won't reappear unless the issue recurs.

## FAQ

### General Questions

**Q: How often do prices update?**
A: Prices update every 60 seconds from each oracle node.

**Q: How accurate are the prices?**
A: Prices are aggregated from 5-10 sources and validated. The confidence score indicates reliability.

**Q: What happens if a node goes offline?**
A: The system continues functioning with remaining nodes. Minimum 3 active nodes required.

**Q: Can I request a new asset to be added?**
A: Yes! Submit requests through our GitHub or Discord.

### Technical Questions

**Q: How do I integrate the oracle into my dApp?**
A: See the [Developer Guide](developer-guide.md) for integration instructions.

**Q: What is TEE and why does it matter?**
A: TEE (Trusted Execution Environment) provides hardware-level security, ensuring oracle nodes run exactly as programmed without tampering.

**Q: How is this different from other oracles?**
A: We use TEE for security instead of traditional crypto-economic incentives, providing a different security model.

**Q: What blockchain is this on?**
A: NEAR Protocol. We may expand to other chains in the future.

### Dashboard Questions

**Q: Why is a price marked as "stale"?**
A: Price data hasn't been updated in over 5 minutes. This usually resolves automatically.

**Q: What does the confidence score mean?**
A: It measures how closely different data sources agree. Higher is better.

**Q: Can I export price data?**
A: Currently not available in the UI. Use the smart contract to query historical data.

**Q: Is there a mobile app?**
A: Not yet, but the dashboard is mobile-responsive.

### Troubleshooting

**Q: Dashboard shows "Loading" forever**
A: Try refreshing the page. Check your internet connection. Clear browser cache.

**Q: Prices look wrong**
A: Check the confidence score. If low, data may be unreliable. Report issues to support.

**Q: Node status not updating**
A: Refresh the page. If issue persists, nodes may be experiencing temporary issues.

## Best Practices

### For Traders

1. **Check Confidence**: Only use prices with >90% confidence
2. **Verify Freshness**: Ensure updates are recent (<2 minutes)
3. **Cross-Reference**: Compare with your exchange prices
4. **Watch Alerts**: Monitor for system notifications

### For Developers

1. **Implement Fallbacks**: Handle cases where prices are unavailable
2. **Cache Appropriately**: Don't query on every user action
3. **Validate Data**: Check timestamps and confidence scores
4. **Monitor Usage**: Track your application's oracle usage

### For Node Operators

1. **Monitor Uptime**: Keep nodes running 24/7
2. **Check Logs**: Review logs regularly for issues
3. **Update Software**: Keep node software current
4. **Respond to Alerts**: Address issues promptly

## Getting Help

### Support Channels

- **Documentation**: https://docs.near-tee-oracle.io
- **GitHub Issues**: https://github.com/near-tee-oracle/issues
- **Discord**: [Community Channel]
- **Email**: support@near-tee-oracle.io

### Reporting Issues

When reporting issues, include:
1. What you were trying to do
2. What happened instead
3. Screenshot (if applicable)
4. Browser and version
5. Time when issue occurred

### Feature Requests

We welcome suggestions! Submit feature requests via:
- GitHub Discussions
- Discord community channel
- Email to feedback@near-tee-oracle.io

## Updates and Maintenance

### Scheduled Maintenance

Maintenance windows are announced 48 hours in advance via:
- Dashboard banner
- Discord announcements
- Email notifications (if subscribed)

Typical duration: 1-2 hours
Typical frequency: Monthly

### System Updates

We release updates regularly:
- **Major updates**: New features, quarterly
- **Minor updates**: Improvements, monthly
- **Patches**: Bug fixes, as needed

Check the changelog: https://github.com/near-tee-oracle/releases

## Privacy and Security

### Data Collection

We collect minimal data:
- Anonymous usage statistics
- Error logs (no personal info)
- Performance metrics

### No Account Required

The dashboard is completely public. No registration, no tracking, no cookies (except essential ones).

### Security

- All data transmitted over HTTPS
- No sensitive information stored
- Open source code for transparency

## Contributing

Want to contribute? We welcome:
- Bug reports
- Feature suggestions
- Documentation improvements
- Code contributions

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

---

Thank you for using NEAR TEE Oracle! If you have questions not covered here, please reach out through our support channels.

