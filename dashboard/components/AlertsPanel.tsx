import { AlertCircle, AlertTriangle, Info, X } from 'lucide-react'
import { useState } from 'react'
import { formatDistanceToNow } from 'date-fns'

interface Alert {
  id: string
  type: 'error' | 'warning' | 'info'
  title: string
  message: string
  timestamp: number
}

interface AlertsPanelProps {
  alerts: Alert[]
}

export default function AlertsPanel({ alerts: initialAlerts }: AlertsPanelProps) {
  const [alerts, setAlerts] = useState(initialAlerts)

  const dismissAlert = (id: string) => {
    setAlerts(alerts.filter(alert => alert.id !== id))
  }

  const getAlertIcon = (type: string) => {
    switch (type) {
      case 'error':
        return <AlertCircle className="w-5 h-5 text-red-500" />
      case 'warning':
        return <AlertTriangle className="w-5 h-5 text-yellow-500" />
      case 'info':
        return <Info className="w-5 h-5 text-blue-500" />
      default:
        return <Info className="w-5 h-5 text-gray-500" />
    }
  }

  const getAlertStyles = (type: string) => {
    switch (type) {
      case 'error':
        return 'bg-red-500/10 border-red-500'
      case 'warning':
        return 'bg-yellow-500/10 border-yellow-500'
      case 'info':
        return 'bg-blue-500/10 border-blue-500'
      default:
        return 'bg-gray-500/10 border-gray-500'
    }
  }

  if (alerts.length === 0) return null

  return (
    <div className="space-y-3">
      <h2 className="text-xl font-bold text-white">Alerts</h2>
      {alerts.map((alert) => (
        <div
          key={alert.id}
          className={`border rounded-lg p-4 flex items-start space-x-3 ${getAlertStyles(alert.type)}`}
        >
          <div className="flex-shrink-0 mt-0.5">
            {getAlertIcon(alert.type)}
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-start justify-between">
              <div>
                <h3 className="font-semibold text-white">{alert.title}</h3>
                <p className="text-sm text-gray-300 mt-1">{alert.message}</p>
                <p className="text-xs text-gray-400 mt-2">
                  {formatDistanceToNow(alert.timestamp, { addSuffix: true })}
                </p>
              </div>
              <button
                onClick={() => dismissAlert(alert.id)}
                className="flex-shrink-0 ml-4 text-gray-400 hover:text-white transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      ))}
    </div>
  )
}

