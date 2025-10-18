'use client'

import { useState, useEffect } from 'react'
import PriceGrid from '@/components/PriceGrid'
import NodeStatus from '@/components/NodeStatus'
import AlertsPanel from '@/components/AlertsPanel'
import StatsOverview from '@/components/StatsOverview'
import Header from '@/components/Header'
import { useOracleData } from '@/hooks/useOracleData'

export default function Home() {
  const { prices, nodes, stats, alerts, loading, error } = useOracleData()

  return (
    <main className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900">
      <Header />
      
      <div className="container mx-auto px-4 py-8 space-y-8">
        {/* Loading State */}
        {loading && (
          <div className="flex items-center justify-center h-64">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
          </div>
        )}

        {/* Error State */}
        {error && (
          <div className="bg-red-500/10 border border-red-500 rounded-lg p-4 text-red-400">
            <p className="font-semibold">Error loading oracle data</p>
            <p className="text-sm">{error}</p>
          </div>
        )}

        {/* Main Content */}
        {!loading && !error && (
          <>
            {/* Stats Overview */}
            <StatsOverview stats={stats} />

            {/* Alerts Panel */}
            {alerts && alerts.length > 0 && (
              <AlertsPanel alerts={alerts} />
            )}

            {/* Price Grid */}
            <section>
              <h2 className="text-2xl font-bold text-white mb-4">Live Prices</h2>
              <PriceGrid prices={prices} />
            </section>

            {/* Node Status */}
            <section>
              <h2 className="text-2xl font-bold text-white mb-4">Oracle Nodes</h2>
              <NodeStatus nodes={nodes} />
            </section>
          </>
        )}
      </div>
    </main>
  )
}

