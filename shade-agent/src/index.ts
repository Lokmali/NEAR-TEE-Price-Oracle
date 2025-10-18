import * as nearAPI from 'near-api-js'
import { Agent, AgentConfig, QueryResult } from './types'

const { connect, keyStores, utils } = nearAPI

export class NearOracleAgent implements Agent {
  private near: nearAPI.Near | null = null
  private account: nearAPI.Account | null = null
  private config: AgentConfig
  private requestCount: Map<string, number> = new Map()

  constructor(config: AgentConfig) {
    this.config = config
  }

  async initialize(): Promise<void> {
    const nearConfig = {
      networkId: this.config.oracle.network,
      keyStore: new keyStores.InMemoryKeyStore(),
      nodeUrl: this.config.oracle.rpc_url,
      walletUrl: `https://wallet.${this.config.oracle.network}.near.org`,
      helperUrl: `https://helper.${this.config.oracle.network}.near.org`,
    }

    this.near = await connect(nearConfig)
    this.account = await this.near.account(this.config.oracle.contract_id)
  }

  async queryPrice(asset: string): Promise<QueryResult> {
    this.checkRateLimit()

    const query = `What is the price of ${asset}?`
    
    try {
      const result = await this.account!.viewFunction({
        contractId: this.config.oracle.contract_id,
        methodName: 'agent_query',
        args: { query },
      })

      return {
        success: true,
        data: result,
        timestamp: Date.now(),
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: Date.now(),
      }
    }
  }

  async queryMultiplePrices(assets: string[]): Promise<QueryResult> {
    this.checkRateLimit()

    try {
      const result = await this.account!.viewFunction({
        contractId: this.config.oracle.contract_id,
        methodName: 'agent_batch_query',
        args: { symbols: assets },
      })

      return {
        success: true,
        data: result,
        timestamp: Date.now(),
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: Date.now(),
      }
    }
  }

  async getSupportedAssets(): Promise<QueryResult> {
    this.checkRateLimit()

    try {
      const result = await this.account!.viewFunction({
        contractId: this.config.oracle.contract_id,
        methodName: 'get_supported_assets',
        args: {},
      })

      return {
        success: true,
        data: result,
        timestamp: Date.now(),
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: Date.now(),
      }
    }
  }

  async getNodeStatus(): Promise<QueryResult> {
    this.checkRateLimit()

    try {
      const result = await this.account!.viewFunction({
        contractId: this.config.oracle.contract_id,
        methodName: 'get_active_nodes',
        args: {},
      })

      return {
        success: true,
        data: result,
        timestamp: Date.now(),
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: Date.now(),
      }
    }
  }

  async processNaturalLanguageQuery(query: string): Promise<string> {
    this.checkRateLimit()

    try {
      const result = await this.account!.viewFunction({
        contractId: this.config.oracle.contract_id,
        methodName: 'agent_query',
        args: { query },
      })

      return result as string
    } catch (error) {
      return `Error processing query: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }

  private checkRateLimit(): void {
    const now = Date.now()
    const minute = Math.floor(now / 60000)
    const key = `requests_${minute}`
    
    const count = this.requestCount.get(key) || 0
    
    if (count >= this.config.rate_limits.requests_per_minute) {
      throw new Error('Rate limit exceeded. Please wait before making more requests.')
    }

    this.requestCount.set(key, count + 1)

    // Clean up old entries
    for (const [k, _] of this.requestCount) {
      const keyMinute = parseInt(k.split('_')[1])
      if (keyMinute < minute - 5) {
        this.requestCount.delete(k)
      }
    }
  }
}

// Example usage
export async function createAgent(configPath: string = './agent-config.json'): Promise<NearOracleAgent> {
  const fs = await import('fs/promises')
  const configData = await fs.readFile(configPath, 'utf-8')
  const config: AgentConfig = JSON.parse(configData)
  
  const agent = new NearOracleAgent(config)
  await agent.initialize()
  
  return agent
}

// CLI interface for the agent
export async function runAgentCLI() {
  const readline = await import('readline')
  
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  })

  console.log('NEAR TEE Oracle Agent - Interactive Mode')
  console.log('=========================================')
  console.log('Ask questions about crypto prices!')
  console.log('Type "exit" to quit\n')

  const agent = await createAgent()

  const processQuery = () => {
    rl.question('You: ', async (query) => {
      if (query.toLowerCase() === 'exit') {
        console.log('Goodbye!')
        rl.close()
        return
      }

      const response = await agent.processNaturalLanguageQuery(query)
      console.log(`Agent: ${response}\n`)
      
      processQuery()
    })
  }

  processQuery()
}

