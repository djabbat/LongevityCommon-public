import React from 'react'
import type { ZeProfile, HealthFactorSummary } from '../../types'

interface Props {
  profile: ZeProfile
}

const stabilityColor: Record<string, string> = {
  high: '#22c55e',
  medium: '#f59e0b',
  low: '#ef4444',
}

export function ZeProfileCard({ profile }: Props) {
  const delta = profile.bio_age_delta
  const deltaStr = delta !== undefined
    ? `${delta > 0 ? '+' : ''}${delta.toFixed(1)}`
    : '—'

  const stability = profile.ci_stability ?? 'low'

  return (
    <div style={{
      background: '#1e293b',
      border: '1px solid #334155',
      borderRadius: 12,
      padding: 24,
      fontFamily: 'monospace',
      color: '#e2e8f0',
      maxWidth: 420,
    }}>
      <div style={{ fontSize: 18, fontWeight: 700, marginBottom: 12, color: '#7dd3fc' }}>
        @{profile.username}
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8, marginBottom: 16 }}>
        <div>
          <div style={{ fontSize: 11, color: '#94a3b8' }}>CHRONO AGE</div>
          <div style={{ fontSize: 24, fontWeight: 700 }}>
            {profile.chrono_age ? Math.round(profile.chrono_age) : '—'}
          </div>
        </div>
        <div>
          <div style={{ fontSize: 11, color: '#94a3b8' }}>BIO AGE EST</div>
          <div style={{ fontSize: 24, fontWeight: 700, color: delta && delta < 0 ? '#22c55e' : '#f87171' }}>
            {profile.bio_age_est !== undefined ? profile.bio_age_est.toFixed(1) : '—'}
          </div>
        </div>
      </div>

      {profile.bio_age_ci_low !== undefined && (
        <div style={{ fontSize: 12, color: '#94a3b8', marginBottom: 8 }}>
          95% CI: [{profile.bio_age_ci_low.toFixed(1)}, {profile.bio_age_ci_high?.toFixed(1)}]
          &nbsp;&nbsp;
          <span style={{ color: delta && delta < 0 ? '#22c55e' : '#f87171' }}>
            {deltaStr} from chrono
          </span>
        </div>
      )}

      <div style={{ fontSize: 12, marginBottom: 16 }}>
        Stability:{' '}
        <span style={{ color: stabilityColor[stability], fontWeight: 600 }}>
          {stability}
        </span>
      </div>

      <div style={{ borderTop: '1px solid #334155', marginTop: 12, paddingTop: 12 }}>
        <div style={{ fontSize: 11, color: '#94a3b8', marginBottom: 8, letterSpacing: '0.05em' }}>
          ORGANISM
        </div>
        <ZeBar label="χ_Ze (EEG)" value={profile.chi_ze_eeg} />
        <ZeBar label="χ_Ze (HRV)" value={profile.chi_ze_hrv} />
      </div>

      <HealthFactorsPanel factors={profile.health_factors} />

      {profile.trend_30d !== undefined && (
        <div style={{ fontSize: 12, marginTop: 12 }}>
          Trend (30d):{' '}
          <span style={{ color: profile.trend_30d >= 0 ? '#22c55e' : '#f87171', fontWeight: 600 }}>
            {profile.trend_30d >= 0 ? '↑' : '↓'} {Math.abs(profile.trend_30d * 1000).toFixed(1)}×10⁻³
          </span>
        </div>
      )}

      {profile.cohort_percentile !== undefined && (
        <div style={{ fontSize: 12, marginTop: 4, color: '#94a3b8' }}>
          Cohort: top {(100 - profile.cohort_percentile).toFixed(0)}% for your age group
        </div>
      )}

      <div style={{ fontSize: 12, marginTop: 8 }}>
        FCLC node:{' '}
        <span style={{ color: profile.fclc_node_active ? '#22c55e' : '#94a3b8' }}>
          {profile.fclc_node_active ? '● active' : '○ inactive'}
        </span>
      </div>
    </div>
  )
}

function ZeBar({ label, value }: { label: string; value?: number }) {
  if (value === undefined) return null
  const pct = Math.round(value * 100)
  return (
    <div style={{ marginBottom: 8 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 12, marginBottom: 2 }}>
        <span>{label}</span>
        <span style={{ color: '#7dd3fc' }}>{value.toFixed(2)}</span>
      </div>
      <div style={{ background: '#334155', borderRadius: 4, height: 6 }}>
        <div style={{
          width: `${pct}%`,
          background: `hsl(${pct * 1.2}, 70%, 55%)`,
          height: '100%',
          borderRadius: 4,
          transition: 'width 0.5s ease',
        }} />
      </div>
    </div>
  )
}

const FACTOR_LABELS: Record<string, string> = {
  psyche: 'ПСИХИКА',
  consciousness: 'СОЗНАНИЕ',
  social: 'СОЦИУМ',
}

function HealthFactorsPanel({ factors }: { factors: HealthFactorSummary }) {
  const rows = [
    { key: 'psyche', value: factors.psyche },
    { key: 'consciousness', value: factors.consciousness },
    { key: 'social', value: factors.social },
  ]

  return (
    <div style={{ borderTop: '1px solid #334155', marginTop: 12, paddingTop: 12 }}>
      {rows.map(({ key, value }) => (
        <ZeBar key={key} label={FACTOR_LABELS[key]} value={value} />
      ))}

      {factors.health_score !== undefined && (
        <div style={{ marginTop: 10 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 12, marginBottom: 4 }}>
            <span style={{ fontWeight: 700, color: '#e2e8f0' }}>HEALTH SCORE</span>
            <span style={{ color: '#fbbf24', fontWeight: 700 }}>{factors.health_score.toFixed(2)}</span>
          </div>
          <div style={{ background: '#334155', borderRadius: 4, height: 8 }}>
            <div style={{
              width: `${Math.round(factors.health_score * 100)}%`,
              background: 'linear-gradient(90deg, #3b82f6, #22c55e)',
              height: '100%',
              borderRadius: 4,
              transition: 'width 0.5s ease',
            }} />
          </div>
          <div style={{ fontSize: 10, color: '#64748b', marginTop: 3 }}>
            {factors.factors_filled}/4 factors · 0.40·org + 0.25·psych + 0.20·mind + 0.15·social
          </div>
        </div>
      )}

      {factors.factors_filled < 2 && (
        <div style={{ fontSize: 11, color: '#64748b', marginTop: 6 }}>
          Log psyche/consciousness/social to see Health Score
        </div>
      )}
    </div>
  )
}
