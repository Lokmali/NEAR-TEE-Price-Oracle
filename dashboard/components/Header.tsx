import { Activity } from 'lucide-react'

export default function Header() {
  return (
    <header className="bg-gray-900/50 backdrop-blur-sm border-b border-gray-700">
      <div className="container mx-auto px-4 py-6">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <div className="bg-blue-500 p-2 rounded-lg">
              <Activity className="w-8 h-8 text-white" />
            </div>
            <div>
              <h1 className="text-2xl font-bold text-white">NEAR TEE Oracle</h1>
              <p className="text-gray-400 text-sm">Real-time Price Monitoring Dashboard</p>
            </div>
          </div>
          
          <div className="flex items-center space-x-4">
            <div className="text-right">
              <p className="text-sm text-gray-400">Network</p>
              <p className="text-white font-semibold">NEAR Mainnet</p>
            </div>
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
              <span className="text-sm text-green-500">Live</span>
            </div>
          </div>
        </div>
      </div>
    </header>
  )
}

