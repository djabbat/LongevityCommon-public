import React, { useState } from 'react'
import {
  LineChart, Line, XAxis, YAxis, Tooltip,
  ResponsiveContainer, CartesianGrid, ReferenceLine,
} from 'recharts'
import { format, parseISO } from 'date-fns'
import type { ZeTrend } from '../../types'

interface Props {
  trend: ZeTrend
  mode?: 'chi_ze' | 'bio_age'
}

export function ZeTrendChart({ trend, mode = 'chi_ze' }: Props) {
  const dataKey = mode === 'chi_ze' ? 'chi_ze_combined' : 'bio_age_est'
  const label = mode === 'chi_ze' ? 'χ_Ze' : 'Biological Age'
  const color = '#7dd3fc'

  const data = trend.points.map(p => ({
    date: format(parseISO(p.date), 'MMM d'),
    value: mode === 'chi_ze' ? p.chi_ze_combined : p.bio_age_est,
  }))

  return (
    <div style={{ background: '#1e293b', borderRadius: 12, padding: 16 }}>
      <div style={{ fontSize: 13, color: '#94a3b8', marginBottom: 8, fontFamily: 'monospace' }}>
        {label} — {trend.period_days}d trend
      </div>
      <ResponsiveContainer width="100%" height={180}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
          <XAxis dataKey="date" tick={{ fontSize: 11, fill: '#94a3b8' }} />
          <YAxis
            domain={mode === 'chi_ze' ? [0, 1] : ['auto', 'auto']}
            tick={{ fontSize: 11, fill: '#94a3b8' }}
          />
          <Tooltip
            contentStyle={{ background: '#0f172a', border: '1px solid #334155', fontSize: 12 }}
            formatter={(v: number) => [v?.toFixed(3), label]}
          />
          <Line
            type="monotone"
            dataKey="value"
            stroke={color}
            strokeWidth={2}
            dot={false}
            activeDot={{ r: 4 }}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}
