import React, { useRef, useState } from 'react'
import { toPng } from 'html-to-image'
import type { ZeProfile } from '../../types'

interface Props {
  profile: ZeProfile
}

/** Renders a shareable Ze·Profile card and exports it as PNG */
export function ZeShareCard({ profile }: Props) {
  const cardRef = useRef<HTMLDivElement>(null)
  const [exporting, setExporting] = useState(false)

  const delta = profile.bio_age_delta
  const deltaStr = delta !== undefined
    ? `${delta > 0 ? '+' : ''}${delta.toFixed(1)} yrs`
    : ''
  const deltaColor = delta !== undefined && delta < 0 ? '#22c55e' : '#f87171'

  const exportPng = async () => {
    if (!cardRef.current || exporting) return
    setExporting(true)
    try {
      const dataUrl = await toPng(cardRef.current, {
        cacheBust: true,
        pixelRatio: 2,
        backgroundColor: '#0f172a',
      })
      const link = document.createElement('a')
      link.download = `ze-profile-${profile.username}.png`
      link.href = dataUrl
      link.click()
    } finally {
      setExporting(false)
    }
  }

  return (
    <div>
      {/* The card that gets exported */}
      <div
        ref={cardRef}
        style={{
          width: 480,
          background: '#0f172a',
          border: '2px solid #334155',
          borderRadius: 16,
          padding: 32,
          fontFamily: 'monospace',
          color: '#e2e8f0',
          boxSizing: 'border-box',
        }}
      >
        {/* Header */}
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', marginBottom: 24 }}>
          <div>
            <div style={{ fontSize: 13, color: '#7dd3fc', fontWeight: 700, letterSpacing: 2 }}>
              LONGEVITYCOMMON
            </div>
            <div style={{ fontSize: 20, fontWeight: 800, color: '#e2e8f0', marginTop: 4 }}>
              @{profile.username}
            </div>
          </div>
          <div style={{ textAlign: 'right' }}>
            <div style={{ fontSize: 10, color: '#475569' }}>Ze·Profile</div>
            <div style={{ fontSize: 10, color: '#475569' }}>
              {new Date().toLocaleDateString('en-US', { month: 'short', year: 'numeric' })}
            </div>
          </div>
        </div>

        {/* Bio age main display */}
        <div style={{
          background: '#1e293b',
          borderRadius: 12,
          padding: '20px 24px',
          marginBottom: 20,
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}>
          <div>
            <div style={{ fontSize: 11, color: '#64748b', marginBottom: 4 }}>BIOLOGICAL AGE</div>
            <div style={{ fontSize: 48, fontWeight: 800, lineHeight: 1, color: '#e2e8f0' }}>
              {profile.bio_age_est !== undefined ? profile.bio_age_est.toFixed(1) : '—'}
            </div>
            {profile.bio_age_ci_low !== undefined && (
              <div style={{ fontSize: 11, color: '#64748b', marginTop: 6 }}>
                95% CI [{profile.bio_age_ci_low.toFixed(1)}, {profile.bio_age_ci_high?.toFixed(1)}]
              </div>
            )}
          </div>
          <div style={{ textAlign: 'right' }}>
            <div style={{ fontSize: 11, color: '#64748b', marginBottom: 4 }}>CHRONO AGE</div>
            <div style={{ fontSize: 32, fontWeight: 700, color: '#94a3b8' }}>
              {profile.chrono_age ? Math.round(profile.chrono_age) : '—'}
            </div>
            {delta !== undefined && (
              <div style={{ fontSize: 18, fontWeight: 700, color: deltaColor, marginTop: 4 }}>
                {deltaStr}
              </div>
            )}
          </div>
        </div>

        {/* Chi Ze bars */}
        <div style={{ marginBottom: 20 }}>
          {profile.chi_ze_eeg !== undefined && (
            <ShareBar label="χ_Ze (EEG)" value={profile.chi_ze_eeg} />
          )}
          {profile.chi_ze_hrv !== undefined && (
            <ShareBar label="χ_Ze (HRV)" value={profile.chi_ze_hrv} />
          )}
        </div>

        {/* Stats row */}
        <div style={{ display: 'flex', gap: 16 }}>
          {profile.trend_30d !== undefined && (
            <StatChip
              label="30d trend"
              value={`${profile.trend_30d >= 0 ? '↑' : '↓'} ${Math.abs(profile.trend_30d * 1000).toFixed(1)}×10⁻³`}
              color={profile.trend_30d >= 0 ? '#22c55e' : '#f87171'}
            />
          )}
          {profile.cohort_percentile !== undefined && (
            <StatChip
              label="cohort"
              value={`top ${(100 - profile.cohort_percentile).toFixed(0)}%`}
              color="#7dd3fc"
            />
          )}
          <StatChip
            label="CI"
            value={profile.ci_stability ?? '—'}
            color={profile.ci_stability === 'high' ? '#22c55e' : profile.ci_stability === 'medium' ? '#f59e0b' : '#ef4444'}
          />
          {profile.fclc_node_active && (
            <StatChip label="FCLC" value="● node" color="#7dd3fc" />
          )}
        </div>

        {/* Footer */}
        <div style={{ marginTop: 24, paddingTop: 16, borderTop: '1px solid #1e293b', fontSize: 10, color: '#334155' }}>
          longevitycommon.app · Ze Theory by J. Tkemaladze · For scientific use only
        </div>
      </div>

      {/* Export button — outside the exported area */}
      <button
        onClick={exportPng}
        disabled={exporting}
        style={{
          marginTop: 12,
          padding: '8px 20px',
          background: exporting ? '#334155' : '#7dd3fc',
          color: '#0f172a',
          border: 'none',
          borderRadius: 8,
          fontWeight: 700,
          fontSize: 13,
          cursor: exporting ? 'not-allowed' : 'pointer',
          fontFamily: 'monospace',
        }}
      >
        {exporting ? 'Exporting…' : '↓ Save as PNG'}
      </button>
    </div>
  )
}

function ShareBar({ label, value }: { label: string; value: number }) {
  const pct = Math.round(value * 100)
  return (
    <div style={{ marginBottom: 10 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 11, marginBottom: 3 }}>
        <span style={{ color: '#94a3b8' }}>{label}</span>
        <span style={{ color: '#7dd3fc', fontWeight: 700 }}>{value.toFixed(3)}</span>
      </div>
      <div style={{ background: '#1e293b', borderRadius: 3, height: 8 }}>
        <div style={{
          width: `${pct}%`,
          background: `hsl(${pct * 1.2}, 65%, 55%)`,
          height: '100%',
          borderRadius: 3,
        }} />
      </div>
    </div>
  )
}

function StatChip({ label, value, color }: { label: string; value: string; color: string }) {
  return (
    <div style={{
      background: '#1e293b',
      borderRadius: 8,
      padding: '6px 12px',
      flex: 1,
      textAlign: 'center',
    }}>
      <div style={{ fontSize: 9, color: '#475569', marginBottom: 2 }}>{label.toUpperCase()}</div>
      <div style={{ fontSize: 12, fontWeight: 700, color }}>{value}</div>
    </div>
  )
}
