export interface AgentConfig {
  agent: {
    name: string
    version: string
    description: string
    capabilities: string[]
  }
  oracle: {
    network: string
    contract_id: string
    rpc_url: string
  }
  endpoints: {
    price_query: string
    batch_query: string
    supported_assets: string
    node_status: string
  }
  rate_limits: {
    requests_per_minute: number
    burst_size: number
  }
}

export interface QueryResult {
  success: boolean
  data?: any
  error?: string
  timestamp: number
}

export interface Agent {
  initialize(): Promise<void>
  queryPrice(asset: string): Promise<QueryResult>
  queryMultiplePrices(assets: string[]): Promise<QueryResult>
  getSupportedAssets(): Promise<QueryResult>
  getNodeStatus(): Promise<QueryResult>
  processNaturalLanguageQuery(query: string): Promise<string>
}

export interface PriceData {
  symbol: string
  price: string
  timestamp: number
  confidence: number
  sources: PriceSource[]
}

export interface PriceSource {
  name: string
  price: string
  timestamp: string
}

export interface OracleNode {
  account_id: string
  tee_type: string
  region: string
  endpoint: string
  last_update: number
  total_updates: number
  is_active: boolean
}

