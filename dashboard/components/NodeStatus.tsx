import { Server, CheckCircle, XCircle, Clock } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

interface Node {
  account_id: string
  tee_type: string
  region: string
  endpoint: string
  last_update: number
  total_updates: number
  is_active: boolean
}

interface NodeStatusProps {
  nodes: Node[]
}

export default function NodeStatus({ nodes }: NodeStatusProps) {
  const getStatusColor = (node: Node) => {
    if (!node.is_active) return 'bg-gray-500'
    const timeSinceUpdate = Date.now() - node.last_update
    if (timeSinceUpdate < 120000) return 'bg-green-500' // < 2 min
    if (timeSinceUpdate < 300000) return 'bg-yellow-500' // < 5 min
    return 'bg-red-500'
  }

  const getStatusIcon = (node: Node) => {
    if (!node.is_active) return <XCircle className="w-5 h-5 text-gray-400" />
    return <CheckCircle className="w-5 h-5 text-green-500" />
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {nodes.map((node) => (
        <div
          key={node.account_id}
          className="bg-gray-800/50 backdrop-blur-sm border border-gray-700 rounded-lg p-6 hover:border-blue-500 transition-all"
        >
          {/* Header */}
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center space-x-2">
              <Server className="w-5 h-5 text-blue-500" />
              <h3 className="text-lg font-semibold text-white">{node.account_id}</h3>
            </div>
            {getStatusIcon(node)}
          </div>

          {/* Status Indicator */}
          <div className="flex items-center space-x-2 mb-4">
            <div className={`w-3 h-3 rounded-full ${getStatusColor(node)} animate-pulse`}></div>
            <span className="text-sm text-gray-400">
              {node.is_active ? 'Active' : 'Inactive'}
            </span>
          </div>

          {/* Details */}
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-400">TEE Type</span>
              <span className="text-white font-semibold uppercase">{node.tee_type}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Region</span>
              <span className="text-white font-semibold">{node.region}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Total Updates</span>
              <span className="text-white font-semibold">{node.total_updates.toLocaleString()}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Last Update</span>
              <span className="text-white font-semibold">
                {node.last_update > 0
                  ? formatDistanceToNow(node.last_update, { addSuffix: true })
                  : 'Never'}
              </span>
            </div>
          </div>

          {/* Endpoint */}
          <div className="mt-4 pt-4 border-t border-gray-700">
            <p className="text-xs text-gray-400 truncate">{node.endpoint}</p>
          </div>
        </div>
      ))}
    </div>
  )
}

