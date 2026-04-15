import React, { useState } from 'react'
import api from '../../hooks/useApi'
import type { PostType } from '../../types'

interface Props {
  onCreated?: () => void
}

const POST_TYPES: { value: PostType; label: string }[] = [
  { value: 'ze_log', label: 'Ze Log' },
  { value: 'science_thread', label: 'Science Thread' },
  { value: 'study_invite', label: 'Study Invite' },
  { value: 'debate', label: 'Debate' },
]

export function CreatePost({ onCreated }: Props) {
  const [open, setOpen] = useState(false)
  const [type, setType] = useState<PostType>('ze_log')
  const [content, setContent] = useState('')
  const [doi, setDoi] = useState('')
  const [codeUrl, setCodeUrl] = useState('')
  const [loading, setLoading] = useState(false)
  const [feedback, setFeedback] = useState<string | null>(null)

  const submit = async () => {
    if (content.length < 10) return
    setLoading(true)
    try {
      const res = await api.post('/posts', {
        type,
        content,
        doi: doi || undefined,
        code_url: codeUrl || undefined,
      })
      setFeedback(
        res.data.doi_verified === false && doi
          ? `Posted. DOI could not be verified — ranking penalty applied.`
          : 'Posted!'
      )
      setContent('')
      setDoi('')
      setCodeUrl('')
      setOpen(false)
      onCreated?.()
    } finally {
      setLoading(false)
    }
  }

  if (!open) {
    return (
      <button
        onClick={() => setOpen(true)}
        style={{
          width: '100%',
          background: '#1e293b',
          border: '1px dashed #334155',
          borderRadius: 10,
          padding: 14,
          color: '#64748b',
          cursor: 'pointer',
          fontSize: 14,
          marginBottom: 16,
        }}
      >
        + Share a Ze log, science finding, or study invite…
      </button>
    )
  }

  return (
    <div style={{
      background: '#1e293b',
      border: '1px solid #334155',
      borderRadius: 10,
      padding: 16,
      marginBottom: 16,
    }}>
      <div style={{ display: 'flex', gap: 8, marginBottom: 12 }}>
        {POST_TYPES.map(pt => (
          <button
            key={pt.value}
            onClick={() => setType(pt.value)}
            style={{
              padding: '4px 12px',
              borderRadius: 99,
              border: '1px solid',
              borderColor: type === pt.value ? '#7dd3fc' : '#334155',
              background: type === pt.value ? '#7dd3fc22' : 'none',
              color: type === pt.value ? '#7dd3fc' : '#94a3b8',
              cursor: 'pointer',
              fontSize: 12,
            }}
          >
            {pt.label}
          </button>
        ))}
      </div>

      <textarea
        value={content}
        onChange={e => setContent(e.target.value)}
        placeholder="Write your post… (min 10 chars)"
        rows={4}
        style={{
          width: '100%',
          background: '#0f172a',
          border: '1px solid #334155',
          borderRadius: 8,
          padding: 10,
          color: '#e2e8f0',
          fontSize: 14,
          resize: 'vertical',
          boxSizing: 'border-box',
        }}
      />

      <div style={{ display: 'flex', gap: 8, marginTop: 8 }}>
        <input
          value={doi}
          onChange={e => setDoi(e.target.value)}
          placeholder="DOI (optional)"
          style={inputStyle}
        />
        <input
          value={codeUrl}
          onChange={e => setCodeUrl(e.target.value)}
          placeholder="Code URL (optional)"
          style={inputStyle}
        />
      </div>

      {feedback && (
        <div style={{ fontSize: 12, color: '#94a3b8', marginTop: 8 }}>{feedback}</div>
      )}

      <div style={{ display: 'flex', gap: 8, marginTop: 12 }}>
        <button
          onClick={submit}
          disabled={loading || content.length < 10}
          style={{
            background: '#7dd3fc',
            color: '#0f172a',
            border: 'none',
            borderRadius: 8,
            padding: '8px 20px',
            cursor: loading ? 'not-allowed' : 'pointer',
            fontWeight: 700,
            fontSize: 14,
          }}
        >
          {loading ? 'Posting…' : 'Post'}
        </button>
        <button
          onClick={() => setOpen(false)}
          style={{
            background: 'none',
            border: '1px solid #334155',
            borderRadius: 8,
            padding: '8px 16px',
            color: '#94a3b8',
            cursor: 'pointer',
            fontSize: 14,
          }}
        >
          Cancel
        </button>
      </div>
    </div>
  )
}

const inputStyle: React.CSSProperties = {
  flex: 1,
  background: '#0f172a',
  border: '1px solid #334155',
  borderRadius: 8,
  padding: '6px 10px',
  color: '#e2e8f0',
  fontSize: 13,
}
