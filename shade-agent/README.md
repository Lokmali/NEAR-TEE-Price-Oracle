# NEAR TEE Oracle - Shade Agent Integration

This package provides Shade Agent Framework integration for the NEAR TEE Oracle, enabling AI agents to query cryptocurrency prices using natural language.

## Installation

```bash
npm install near-tee-oracle-shade-agent
```

## Quick Start

```typescript
import { createAgent } from 'near-tee-oracle-shade-agent'

// Initialize the agent
const agent = await createAgent()

// Query price using natural language
const response = await agent.processNaturalLanguageQuery('What is the price of Bitcoin?')
console.log(response)
// Output: "The current price of BTC is $67,234.56 (confidence: 98%, updated 1 minute ago)"

// Query multiple prices
const prices = await agent.queryMultiplePrices(['BTC', 'ETH', 'NEAR'])
console.log(prices)
```

## Interactive CLI

Run the interactive CLI to chat with the oracle:

```bash
npm run cli
```

Example conversation:
```
You: What is the price of Ethereum?
Agent: The current price of ETH is $3,456.78 (confidence: 97%, updated 45 seconds ago)

You: Give me prices for BTC, ETH, and NEAR
Agent: [Batch results with all three prices]
```

## Features

- **Natural Language Queries**: Ask questions in plain English
- **Batch Queries**: Get multiple prices at once
- **Rate Limiting**: Built-in rate limiting for production use
- **Type Safety**: Full TypeScript support
- **Error Handling**: Graceful error handling and retries

## API Reference

### `createAgent(configPath?: string): Promise<NearOracleAgent>`

Creates and initializes an oracle agent.

### `agent.queryPrice(asset: string): Promise<QueryResult>`

Query price for a single asset.

```typescript
const result = await agent.queryPrice('BTC')
```

### `agent.queryMultiplePrices(assets: string[]): Promise<QueryResult>`

Query prices for multiple assets.

```typescript
const result = await agent.queryMultiplePrices(['BTC', 'ETH', 'NEAR'])
```

### `agent.processNaturalLanguageQuery(query: string): Promise<string>`

Process a natural language query.

```typescript
const response = await agent.processNaturalLanguageQuery('How much is Bitcoin worth?')
```

### `agent.getSupportedAssets(): Promise<QueryResult>`

Get list of supported assets.

### `agent.getNodeStatus(): Promise<QueryResult>`

Get status of oracle nodes.

## Configuration

Create an `agent-config.json` file:

```json
{
  "agent": {
    "name": "NEAR TEE Oracle Agent",
    "version": "1.0.0",
    "capabilities": ["price_query", "batch_price_query", "node_status"]
  },
  "oracle": {
    "network": "mainnet",
    "contract_id": "oracle.near",
    "rpc_url": "https://rpc.mainnet.near.org"
  },
  "rate_limits": {
    "requests_per_minute": 60,
    "burst_size": 10
  }
}
```

## Use Cases

### Trading Bots
```typescript
const agent = await createAgent()
const btcPrice = await agent.queryPrice('BTC')
// Use price data for trading decisions
```

### Portfolio Trackers
```typescript
const assets = ['BTC', 'ETH', 'NEAR', 'SOL']
const prices = await agent.queryMultiplePrices(assets)
// Calculate portfolio value
```

### Price Alerts
```typescript
setInterval(async () => {
  const price = await agent.queryPrice('ETH')
  if (price.data.price > threshold) {
    sendAlert('ETH price alert!')
  }
}, 60000)
```

### AI Assistants
```typescript
// Integrate with your AI assistant
const userQuery = "Should I buy Bitcoin now?"
const priceInfo = await agent.processNaturalLanguageQuery('What is the price of BTC?')
// Use priceInfo in your AI's response
```

## License

MIT

