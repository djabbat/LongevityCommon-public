import React from 'react'
import { useQuery } from '@tanstack/react-query'
import api from '../hooks/useApi'
import type { Study } from '../types'

export function Studies() {
  const { data: studies, isLoading } = useQuery<Study[]>({
    queryKey: ['studies'],
    queryFn: async () => {
      const { data } = await api.get('/studies?status=recruiting')
      return data
    },
  })

  return (
    <div style={{ maxWidth: 740, margin: '0 auto', padding: 16 }}>
      <div style={{ fontSize: 20, fontWeight: 700, color: '#7dd3fc', marginBottom: 4, fontFamily: 'monospace' }}>
        Lab — Citizen Science
      </div>
      <div style={{ fontSize: 13, color: '#64748b', marginBottom: 20 }}>
        Join a study. Contribute your Ze data. Get listed as co-author.
      </div>

      {isLoading ? (
        <div style={{ color: '#64748b', textAlign: 'center', padding: 40 }}>Loading studies…</div>
      ) : studies?.length === 0 ? (
        <div style={{ color: '#64748b', textAlign: 'center', padding: 40 }}>
          No recruiting studies right now. Check back soon.
        </div>
      ) : (
        studies?.map(study => <StudyCard key={study.id} study={study} />)
      )}
    </div>
  )
}

function StudyCard({ study }: { study: Study }) {
  const [joining, setJoining] = React.useState(false)
  const [joined, setJoined] = React.useState(false)

  const join = async () => {
    setJoining(true)
    try {
      await api.post(`/studies/${study.id}/join`, {
        consent_text: `I consent to participate in "${study.title}" under the FCLC DUA terms. I understand this is research, not medical care. I can withdraw at any time.`,
      })
      setJoined(true)
    } catch {
      // handle error
    } finally {
      setJoining(false)
    }
  }

  const pct = Math.round((study.enrolled_n / study.target_n) * 100)

  return (
    <div style={{
      background: '#1e293b',
      border: '1px solid #334155',
      borderRadius: 10,
      padding: 20,
      marginBottom: 12,
    }}>
      <div style={{ fontSize: 16, fontWeight: 700, color: '#e2e8f0', marginBottom: 6 }}>
        {study.title}
      </div>
      <div style={{ fontSize: 13, color: '#94a3b8', marginBottom: 12, lineHeight: 1.5 }}>
        {study.hypothesis}
      </div>

      <div style={{ display: 'flex', gap: 16, fontSize: 12, color: '#64748b', marginBottom: 12 }}>
        <span>n={study.target_n}</span>
        <span>{study.duration_days}d duration</span>
        <span style={{ color: study.enrolled_n >= study.target_n ? '#f87171' : '#22c55e' }}>
          {study.enrolled_n}/{study.target_n} enrolled
        </span>
      </div>

      <div style={{ background: '#0f172a', borderRadius: 4, height: 6, marginBottom: 12 }}>
        <div style={{ width: `${pct}%`, background: pct >= 100 ? '#f87171' : '#22c55e', height: '100%', borderRadius: 4 }} />
      </div>

      {joined ? (
        <div style={{ fontSize: 13, color: '#22c55e', fontWeight: 600 }}>
          ✓ Enrolled — your consent is recorded
        </div>
      ) : (
        <button
          onClick={join}
          disabled={joining || study.enrolled_n >= study.target_n}
          style={{
            background: '#22c55e',
            color: '#0f172a',
            border: 'none',
            borderRadius: 8,
            padding: '8px 20px',
            cursor: 'pointer',
            fontWeight: 700,
            fontSize: 14,
          }}
        >
          {joining ? 'Joining…' : 'Join Study →'}
        </button>
      )}
    </div>
  )
}
