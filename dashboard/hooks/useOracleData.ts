import { useState, useEffect } from 'react'
import * as nearAPI from 'near-api-js'

const { connect, keyStores } = nearAPI

interface PriceData {
  symbol: string
  price: number
  change24h: number
  confidence: number
  timestamp: number
  sources: number
}

interface Node {
  account_id: string
  tee_type: string
  region: string
  endpoint: string
  last_update: number
  total_updates: number
  is_active: boolean
}

interface Stats {
  totalAssets: number
  activeNodes: number
  avgConfidence: number
  uptime: number
}

interface Alert {
  id: string
  type: 'error' | 'warning' | 'info'
  title: string
  message: string
  timestamp: number
}

interface OracleData {
  prices: PriceData[]
  nodes: Node[]
  stats: Stats
  alerts: Alert[]
  loading: boolean
  error: string | null
}

export function useOracleData(): OracleData {
  const [prices, setPrices] = useState<PriceData[]>([])
  const [nodes, setNodes] = useState<Node[]>([])
  const [stats, setStats] = useState<Stats>({
    totalAssets: 0,
    activeNodes: 0,
    avgConfidence: 0,
    uptime: 99.8,
  })
  const [alerts, setAlerts] = useState<Alert[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetchOracleData()
    
    // Refresh every 30 seconds
    const interval = setInterval(fetchOracleData, 30000)
    return () => clearInterval(interval)
  }, [])

  const fetchOracleData = async () => {
    try {
      // In production, this would connect to NEAR and fetch real data
      // For now, we'll use mock data
      
      // Simulate API delay
      await new Promise(resolve => setTimeout(resolve, 1000))

      // Mock price data
      const mockPrices: PriceData[] = [
        {
          symbol: 'BTC',
          price: 67234.56,
          change24h: 2.34,
          confidence: 98,
          timestamp: Date.now() - 45000,
          sources: 10,
        },
        {
          symbol: 'ETH',
          price: 3456.78,
          change24h: -1.23,
          confidence: 97,
          timestamp: Date.now() - 45000,
          sources: 10,
        },
        {
          symbol: 'NEAR',
          price: 8.92,
          change24h: 5.67,
          confidence: 96,
          timestamp: Date.now() - 45000,
          sources: 9,
        },
        {
          symbol: 'BNB',
          price: 523.45,
          change24h: 1.89,
          confidence: 95,
          timestamp: Date.now() - 45000,
          sources: 9,
        },
        {
          symbol: 'SOL',
          price: 156.78,
          change24h: -2.45,
          confidence: 97,
          timestamp: Date.now() - 45000,
          sources: 10,
        },
        {
          symbol: 'ADA',
          price: 0.67,
          change24h: 0.89,
          confidence: 94,
          timestamp: Date.now() - 45000,
          sources: 8,
        },
        {
          symbol: 'DOT',
          price: 12.34,
          change24h: -0.56,
          confidence: 95,
          timestamp: Date.now() - 45000,
          sources: 9,
        },
        {
          symbol: 'MATIC',
          price: 1.23,
          change24h: 3.45,
          confidence: 93,
          timestamp: Date.now() - 45000,
          sources: 8,
        },
      ]

      // Mock node data
      const mockNodes: Node[] = [
        {
          account_id: 'oracle-node-1.near',
          tee_type: 'sgx',
          region: 'us-east-1',
          endpoint: 'https://node1.oracle.near',
          last_update: Date.now() - 60000,
          total_updates: 15234,
          is_active: true,
        },
        {
          account_id: 'oracle-node-2.near',
          tee_type: 'sgx',
          region: 'eu-west-1',
          endpoint: 'https://node2.oracle.near',
          last_update: Date.now() - 45000,
          total_updates: 15198,
          is_active: true,
        },
        {
          account_id: 'oracle-node-3.near',
          tee_type: 'phala',
          region: 'ap-southeast-1',
          endpoint: 'https://node3.oracle.near',
          last_update: Date.now() - 55000,
          total_updates: 15221,
          is_active: true,
        },
      ]

      // Mock alerts
      const mockAlerts: Alert[] = []
      
      // Add alert if any node hasn't updated recently
      mockNodes.forEach(node => {
        const timeSinceUpdate = Date.now() - node.last_update
        if (timeSinceUpdate > 180000) { // > 3 minutes
          mockAlerts.push({
            id: `node-${node.account_id}`,
            type: 'warning',
            title: 'Node Update Delay',
            message: `${node.account_id} hasn't updated prices in over 3 minutes`,
            timestamp: Date.now(),
          })
        }
      })

      setPrices(mockPrices)
      setNodes(mockNodes)
      setStats({
        totalAssets: mockPrices.length,
        activeNodes: mockNodes.filter(n => n.is_active).length,
        avgConfidence: Math.round(
          mockPrices.reduce((acc, p) => acc + p.confidence, 0) / mockPrices.length
        ),
        uptime: 99.8,
      })
      setAlerts(mockAlerts)
      setLoading(false)
      setError(null)
    } catch (err) {
      console.error('Error fetching oracle data:', err)
      setError(err instanceof Error ? err.message : 'Unknown error')
      setLoading(false)
    }
  }

  return { prices, nodes, stats, alerts, loading, error }
}

// For production, use this function to connect to NEAR
export async function connectToNEAR(networkId: string, contractId: string) {
  const config = {
    networkId,
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: `https://rpc.${networkId}.near.org`,
    walletUrl: `https://wallet.${networkId}.near.org`,
    helperUrl: `https://helper.${networkId}.near.org`,
  }

  const near = await connect(config)
  const account = await near.account(contractId)
  
  return { near, account }
}

