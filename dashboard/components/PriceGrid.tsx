import { TrendingUp, TrendingDown, Minus } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

interface PriceData {
  symbol: string
  price: number
  change24h: number
  confidence: number
  timestamp: number
  sources: number
}

interface PriceGridProps {
  prices: PriceData[]
}

export default function PriceGrid({ prices }: PriceGridProps) {
  const getChangeIcon = (change: number) => {
    if (change > 0) return <TrendingUp className="w-4 h-4 text-green-500" />
    if (change < 0) return <TrendingDown className="w-4 h-4 text-red-500" />
    return <Minus className="w-4 h-4 text-gray-500" />
  }

  const getConfidenceColor = (confidence: number) => {
    if (confidence >= 95) return 'text-green-500'
    if (confidence >= 85) return 'text-yellow-500'
    return 'text-red-500'
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
      {prices.map((price) => (
        <div
          key={price.symbol}
          className="bg-gray-800/50 backdrop-blur-sm border border-gray-700 rounded-lg p-6 hover:border-blue-500 transition-all"
        >
          {/* Header */}
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-xl font-bold text-white">{price.symbol}</h3>
            <div className="flex items-center space-x-1">
              {getChangeIcon(price.change24h)}
              <span
                className={`text-sm font-semibold ${
                  price.change24h > 0
                    ? 'text-green-500'
                    : price.change24h < 0
                    ? 'text-red-500'
                    : 'text-gray-500'
                }`}
              >
                {price.change24h > 0 ? '+' : ''}
                {price.change24h.toFixed(2)}%
              </span>
            </div>
          </div>

          {/* Price */}
          <div className="mb-4">
            <p className="text-3xl font-bold text-white">
              ${price.price.toLocaleString(undefined, {
                minimumFractionDigits: 2,
                maximumFractionDigits: 2,
              })}
            </p>
          </div>

          {/* Metadata */}
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-400">Confidence</span>
              <span className={`font-semibold ${getConfidenceColor(price.confidence)}`}>
                {price.confidence}%
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Sources</span>
              <span className="text-white font-semibold">{price.sources}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Updated</span>
              <span className="text-white font-semibold">
                {formatDistanceToNow(price.timestamp, { addSuffix: true })}
              </span>
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}

