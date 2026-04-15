import React, { useState } from 'react'
import { useZeProfile, useZeTrend } from '../hooks/useZeProfile'
import { ZeProfileCard } from '../components/ui/ZeProfileCard'
import { ZeShareCard } from '../components/ui/ZeShareCard'
import { ZeTrendChart } from '../components/ui/ZeTrendChart'
import { ZeGuide } from '../components/lab/ZeGuide'
import api from '../hooks/useApi'
import type { CreateHealthFactorRequest, PsycheMood } from '../types'

export function Dashboard() {
  const { data: profile, isLoading, refetch } = useZeProfile()
  const logHealthFactor = async (req: CreateHealthFactorRequest) => {
    await api.post('/health-factors', req)
    refetch()
  }
  const [period, setPeriod] = useState<7 | 30 | 365>(30)
  const [chartMode, setChartMode] = useState<'chi_ze' | 'bio_age'>('chi_ze')
  const { data: trend } = useZeTrend(period)
  const [showGuide, setShowGuide] = useState(false)
  const [showShare, setShowShare] = useState(false)
  const [showFactorLog, setShowFactorLog] = useState(false)

  if (isLoading) {
    return <div style={{ color: '#64748b', textAlign: 'center', padding: 60 }}>Loading Ze·Profile…</div>
  }

  return (
    <div style={{ maxWidth: 800, margin: '0 auto', padding: 16 }}>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16, marginBottom: 24 }}>
        <div>
          {profile ? (
            <ZeProfileCard profile={profile} />
          ) : (
            <EmptyProfile />
          )}
        </div>

        <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
          {profile && (
            <div style={{ background: '#1e293b', borderRadius: 12, padding: 16 }}>
              <div style={{ fontSize: 13, color: '#94a3b8', marginBottom: 8 }}>
                Samples: {profile.sample_count} &nbsp;|&nbsp;
                {profile.last_sample_at
                  ? `Last: ${new Date(profile.last_sample_at).toLocaleDateString()}`
                  : 'No data yet'}
              </div>
              <ImportDataHint />
              <button
                onClick={() => setShowShare(!showShare)}
                style={{
                  marginTop: 12,
                  padding: '6px 14px',
                  background: showShare ? '#334155' : '#1e293b',
                  border: '1px solid #334155',
                  borderRadius: 8,
                  color: '#7dd3fc',
                  fontSize: 12,
                  cursor: 'pointer',
                  fontWeight: 600,
                }}
              >
                {showShare ? 'Hide share card' : '↗ Share Ze·Profile'}
              </button>
            </div>
          )}
        </div>
      </div>

      {showShare && profile && (
        <div style={{ marginBottom: 24 }}>
          <ZeShareCard profile={profile} />
        </div>
      )}

      {trend && trend.points.length > 0 && (
        <div style={{ marginBottom: 24 }}>
          <div style={{ display: 'flex', gap: 8, marginBottom: 8 }}>
            {([7, 30, 365] as const).map(p => (
              <button
                key={p}
                onClick={() => setPeriod(p)}
                style={periodBtnStyle(period === p)}
              >
                {p === 365 ? '1y' : `${p}d`}
              </button>
            ))}
            <span style={{ marginLeft: 'auto' }}>
              {(['chi_ze', 'bio_age'] as const).map(m => (
                <button
                  key={m}
                  onClick={() => setChartMode(m)}
                  style={periodBtnStyle(chartMode === m)}
                >
                  {m === 'chi_ze' ? 'χ_Ze' : 'Bio Age'}
                </button>
              ))}
            </span>
          </div>
          <ZeTrendChart trend={trend} mode={chartMode} />
        </div>
      )}

      {/* Health Factor Logger */}
      <div style={{ background: '#1e293b', borderRadius: 12, overflow: 'hidden', border: '1px solid #334155', marginBottom: 16 }}>
        <div
          onClick={() => setShowFactorLog(!showFactorLog)}
          style={{ padding: '12px 16px', cursor: 'pointer', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}
        >
          <span style={{ fontWeight: 700, color: '#a78bfa' }}>Log Health Factors</span>
          <span style={{ color: '#94a3b8', fontSize: 11 }}>
            {profile?.health_factors.factors_filled ?? 0}/4 today &nbsp;{showFactorLog ? '▲' : '▼'}
          </span>
        </div>
        {showFactorLog && (
          <HealthFactorForm
            onSubmit={async (req) => {
              await logHealthFactor(req)
              setShowFactorLog(false)
            }}
          />
        )}
      </div>

      {/* Ze·Guide */}
      <div style={{ background: '#1e293b', borderRadius: 12, overflow: 'hidden', border: '1px solid #334155' }}>
        <div
          onClick={() => setShowGuide(!showGuide)}
          style={{ padding: '12px 16px', cursor: 'pointer', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}
        >
          <span style={{ fontWeight: 700, color: '#7dd3fc' }}>Ze·Guide</span>
          <span style={{ color: '#94a3b8' }}>{showGuide ? '▲' : '▼'}</span>
        </div>
        {showGuide && <ZeGuide />}
      </div>
    </div>
  )
}

function EmptyProfile() {
  return (
    <div style={{
      background: '#1e293b',
      border: '1px dashed #334155',
      borderRadius: 12,
      padding: 24,
      color: '#64748b',
      fontSize: 14,
      textAlign: 'center',
    }}>
      <div style={{ fontSize: 32, marginBottom: 8 }}>⟳</div>
      No Ze data yet.
      <br />
      Import your first dataset to see your Ze·Profile.
    </div>
  )
}

function ImportDataHint() {
  return (
    <div style={{ fontSize: 12, color: '#64748b' }}>
      Import data: Settings → Import JSON<br />
      (BioSense · Oura · Garmin · Apple Health)
    </div>
  )
}

const periodBtnStyle = (active: boolean): React.CSSProperties => ({
  padding: '4px 10px',
  border: '1px solid',
  borderColor: active ? '#7dd3fc' : '#334155',
  background: active ? '#7dd3fc22' : 'none',
  color: active ? '#7dd3fc' : '#94a3b8',
  borderRadius: 6,
  cursor: 'pointer',
  fontSize: 12,
})

// ── Health Factor Logger Form ──────────────────────────────────────────────────

const MOOD_OPTIONS: { value: PsycheMood; label: string }[] = [
  { value: 'very_good', label: '😄 Very good' },
  { value: 'good',      label: '🙂 Good' },
  { value: 'neutral',   label: '😐 Neutral' },
  { value: 'bad',       label: '😔 Bad' },
  { value: 'very_bad',  label: '😢 Very bad' },
]

const MOOD_SCORE: Record<PsycheMood, number> = {
  very_good: 1.0, good: 0.75, neutral: 0.5, bad: 0.25, very_bad: 0.0,
}

function SliderRow({
  label, value, onChange,
}: { label: string; value: number; onChange: (v: number) => void }) {
  return (
    <div style={{ marginBottom: 10 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 12, marginBottom: 3 }}>
        <span style={{ color: '#94a3b8' }}>{label}</span>
        <span style={{ color: '#7dd3fc' }}>{value.toFixed(2)}</span>
      </div>
      <input
        type="range" min={0} max={1} step={0.01}
        value={value}
        onChange={e => onChange(parseFloat(e.target.value))}
        style={{ width: '100%', accentColor: '#7dd3fc' }}
      />
    </div>
  )
}

function HealthFactorForm({ onSubmit }: { onSubmit: (r: CreateHealthFactorRequest) => Promise<void> }) {
  const [mood, setMood] = useState<PsycheMood>('neutral')
  const [stress, setStress] = useState(0.3)
  const [mindful, setMindful] = useState(0.5)
  const [purpose, setPurpose] = useState(0.5)
  const [support, setSupport] = useState(0.5)
  const [isolation, setIsolation] = useState(0.3)
  const [submitting, setSubmitting] = useState(false)

  const handleSubmit = async () => {
    setSubmitting(true)
    try {
      const psycheScore = (MOOD_SCORE[mood] + (1 - stress)) / 2
      const consciousnessScore = (mindful + purpose) / 2
      const socialScore = (support + (1 - isolation)) / 2
      await onSubmit({
        recorded_at: new Date().toISOString(),
        psyche_score: psycheScore,
        psyche_mood: mood,
        psyche_stress: stress,
        consciousness_score: consciousnessScore,
        consciousness_mindful: mindful,
        consciousness_purpose: purpose,
        social_score: socialScore,
        social_support: support,
        social_isolation: isolation,
      })
    } finally {
      setSubmitting(false)
    }
  }

  const inputStyle: React.CSSProperties = {
    padding: '6px 10px',
    background: '#0f172a',
    border: '1px solid #334155',
    borderRadius: 6,
    color: '#e2e8f0',
    fontSize: 12,
    cursor: 'pointer',
  }

  return (
    <div style={{ padding: '0 16px 16px', fontSize: 13 }}>
      {/* Psyche */}
      <div style={{ marginBottom: 14 }}>
        <div style={{ color: '#a78bfa', fontWeight: 700, fontSize: 11, marginBottom: 8, letterSpacing: '0.05em' }}>
          ПСИХИКА
        </div>
        <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap', marginBottom: 8 }}>
          {MOOD_OPTIONS.map(o => (
            <button
              key={o.value}
              onClick={() => setMood(o.value)}
              style={{ ...inputStyle, borderColor: mood === o.value ? '#a78bfa' : '#334155', background: mood === o.value ? '#a78bfa22' : '#0f172a' }}
            >
              {o.label}
            </button>
          ))}
        </div>
        <SliderRow label="Stress level" value={stress} onChange={setStress} />
      </div>

      {/* Consciousness */}
      <div style={{ marginBottom: 14 }}>
        <div style={{ color: '#34d399', fontWeight: 700, fontSize: 11, marginBottom: 8, letterSpacing: '0.05em' }}>
          СОЗНАНИЕ
        </div>
        <SliderRow label="Mindfulness" value={mindful} onChange={setMindful} />
        <SliderRow label="Sense of purpose" value={purpose} onChange={setPurpose} />
      </div>

      {/* Social */}
      <div style={{ marginBottom: 16 }}>
        <div style={{ color: '#fbbf24', fontWeight: 700, fontSize: 11, marginBottom: 8, letterSpacing: '0.05em' }}>
          СОЦИУМ
        </div>
        <SliderRow label="Social support" value={support} onChange={setSupport} />
        <SliderRow label="Isolation" value={isolation} onChange={setIsolation} />
      </div>

      <button
        onClick={handleSubmit}
        disabled={submitting}
        style={{
          width: '100%', padding: '8px 0',
          background: submitting ? '#334155' : '#6d28d9',
          border: 'none', borderRadius: 8,
          color: '#fff', fontWeight: 700, fontSize: 13,
          cursor: submitting ? 'default' : 'pointer',
        }}
      >
        {submitting ? 'Saving…' : 'Save health factors'}
      </button>
    </div>
  )
}
