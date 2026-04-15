import React, { useState, useRef, useEffect } from 'react'
import api from '../../hooks/useApi'
import type { ZeGuideResponse } from '../../types'

interface Message {
  role: 'user' | 'guide'
  content: string
  cited_dois?: string[]
  model_used?: string
}

export function ZeGuide() {
  const [messages, setMessages] = useState<Message[]>([])
  const [input, setInput] = useState('')
  const [sessionId, setSessionId] = useState<string | undefined>()
  const [loading, setLoading] = useState(false)
  const [disclaimerAcknowledged, setDisclaimerAcknowledged] = useState(false)
  const bottomRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  const send = async () => {
    const prompt = input.trim()
    if (!prompt || loading) return

    setMessages(prev => [...prev, { role: 'user', content: prompt }])
    setInput('')
    setLoading(true)

    try {
      const { data } = await api.post<ZeGuideResponse>('/ze-guide/ask', {
        prompt,
        session_id: sessionId,
      })
      setSessionId(data.session_id)
      setMessages(prev => [
        ...prev,
        {
          role: 'guide',
          content: data.response,
          cited_dois: data.cited_dois,
          model_used: data.model_used,
        },
      ])
    } catch {
      setMessages(prev => [
        ...prev,
        { role: 'guide', content: 'Ze·Guide is temporarily unavailable. Please try again.' },
      ])
    } finally {
      setLoading(false)
    }
  }

  // Block chat until user explicitly acknowledges disclaimer
  if (!disclaimerAcknowledged) {
    return (
      <div style={{ padding: 24, display: 'flex', flexDirection: 'column', gap: 16 }}>
        <div style={{
          background: '#1c1917',
          border: '1px solid #f59e0b',
          borderRadius: 10,
          padding: 20,
        }}>
          <div style={{ fontSize: 14, fontWeight: 700, color: '#f59e0b', marginBottom: 10 }}>
            ⚠ Before using Ze·Guide
          </div>
          <div style={{ fontSize: 13, color: '#d1d5db', lineHeight: 1.7 }}>
            Ze·Guide is a <strong>scientific research assistant</strong>, not a physician or medical device.<br /><br />
            • χ_Ze and D_norm are <strong>research metrics</strong>, not diagnostic tools<br />
            • Ze·Guide cannot diagnose, treat, or prevent any disease<br />
            • All health decisions must be made with a <strong>licensed medical specialist</strong><br />
            • Every conversation is logged for legal compliance (GDPR Art. 6(1)(a))<br /><br />
            By clicking "I understand", you confirm you are using Ze·Guide for <strong>scientific information only</strong>.
          </div>
        </div>
        <button
          onClick={() => setDisclaimerAcknowledged(true)}
          style={{
            background: '#7dd3fc',
            color: '#0f172a',
            border: 'none',
            borderRadius: 8,
            padding: '12px 24px',
            fontWeight: 700,
            fontSize: 14,
            cursor: 'pointer',
            alignSelf: 'flex-start',
          }}
        >
          I understand — open Ze·Guide
        </button>
      </div>
    )
  }

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', maxHeight: 600 }}>
      <div style={{ padding: '8px 16px', background: '#1e293b', borderBottom: '1px solid #334155', fontSize: 13, color: '#94a3b8' }}>
        <span style={{ color: '#7dd3fc', fontWeight: 700 }}>Ze·Guide</span>
        {' '}— scientific assistant · not a physician
      </div>

      {messages.length === 0 && (
        <div style={{ padding: 24, color: '#64748b', fontSize: 13, fontStyle: 'italic' }}>
          Ask anything about χ_Ze, D_norm, biological age, FCLC, or your personal data trends.
          <br /><br />
          Examples:<br />
          • "Why did my χ_Ze drop this week?"<br />
          • "What does D_norm = 0.15 mean for someone my age?"<br />
          • "Summarize Ze Vectors Theory in 3 sentences"
        </div>
      )}

      <div style={{ flex: 1, overflowY: 'auto', padding: 12 }}>
        {messages.map((msg, i) => (
          <div key={i} style={{
            marginBottom: 12,
            display: 'flex',
            flexDirection: 'column',
            alignItems: msg.role === 'user' ? 'flex-end' : 'flex-start',
          }}>
            <div style={{
              background: msg.role === 'user' ? '#0369a1' : '#1e293b',
              border: msg.role === 'guide' ? '1px solid #334155' : 'none',
              borderRadius: 10,
              padding: '10px 14px',
              maxWidth: '85%',
              fontSize: 14,
              color: '#e2e8f0',
              whiteSpace: 'pre-wrap',
              lineHeight: 1.6,
            }}>
              {msg.role === 'guide' && (
                <div style={{
                  fontSize: 11,
                  color: '#94a3b8',
                  marginBottom: 6,
                  fontStyle: 'italic',
                  borderBottom: '1px solid #334155',
                  paddingBottom: 6,
                }}>
                  Ze·Guide — scientific assistant, not a physician
                </div>
              )}
              {msg.content}
              {msg.cited_dois && msg.cited_dois.length > 0 && (
                <div style={{ marginTop: 8, fontSize: 12, color: '#7dd3fc' }}>
                  Sources: {msg.cited_dois.join(', ')}
                </div>
              )}
            </div>
            {msg.model_used && (
              <div style={{ fontSize: 10, color: '#475569', marginTop: 2 }}>
                {msg.model_used}
              </div>
            )}
          </div>
        ))}
        {loading && (
          <div style={{ color: '#7dd3fc', fontSize: 13, fontStyle: 'italic', padding: 8 }}>
            Ze·Guide is thinking…
          </div>
        )}
        <div ref={bottomRef} />
      </div>

      <div style={{ padding: 12, borderTop: '1px solid #334155', display: 'flex', gap: 8 }}>
        <input
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && !e.shiftKey && send()}
          placeholder="Ask Ze·Guide…"
          disabled={loading}
          style={{
            flex: 1,
            background: '#0f172a',
            border: '1px solid #334155',
            borderRadius: 8,
            padding: '8px 12px',
            color: '#e2e8f0',
            fontSize: 14,
          }}
        />
        <button
          onClick={send}
          disabled={loading || !input.trim()}
          style={{
            background: '#7dd3fc',
            color: '#0f172a',
            border: 'none',
            borderRadius: 8,
            padding: '8px 16px',
            cursor: loading ? 'not-allowed' : 'pointer',
            fontWeight: 700,
          }}
        >
          →
        </button>
      </div>
    </div>
  )
}
