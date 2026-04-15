import React, { useRef, useState } from 'react'
import api from '../hooks/useApi'

type ImportStatus = 'idle' | 'uploading' | 'success' | 'error'

export function Settings() {
  const fileInputRef = useRef<HTMLInputElement>(null)
  const [dragOver, setDragOver] = useState(false)
  const [status, setStatus] = useState<ImportStatus>('idle')
  const [message, setMessage] = useState<string | null>(null)

  const handleFile = async (file: File) => {
    if (!file.name.endsWith('.json')) {
      setStatus('error')
      setMessage('Only JSON files are supported.')
      return
    }

    setStatus('uploading')
    setMessage(null)

    try {
      const text = await file.text()
      const payload = JSON.parse(text)
      await api.post('/data/import', payload)
      setStatus('success')
      setMessage(`Imported successfully from ${file.name}.`)
    } catch (e: unknown) {
      setStatus('error')
      const msg = e instanceof Error ? e.message : 'Import failed. Check file format.'
      setMessage(msg)
    }
  }

  const onDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault()
    setDragOver(false)
    const file = e.dataTransfer.files[0]
    if (file) handleFile(file)
  }

  const onFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) handleFile(file)
    e.target.value = ''
  }

  const statusColor = {
    idle: '#334155',
    uploading: '#f59e0b',
    success: '#22c55e',
    error: '#ef4444',
  }[status]

  return (
    <div style={{ maxWidth: 600, margin: '0 auto', padding: 16 }}>
      <h2 style={{ color: '#e2e8f0', fontWeight: 700, marginBottom: 24, fontSize: 20 }}>
        Settings
      </h2>

      {/* Import section */}
      <section style={{ marginBottom: 32 }}>
        <div style={{ color: '#94a3b8', fontSize: 13, marginBottom: 16, fontWeight: 600 }}>
          IMPORT ZE DATA
        </div>

        <div
          onDragOver={e => { e.preventDefault(); setDragOver(true) }}
          onDragLeave={() => setDragOver(false)}
          onDrop={onDrop}
          onClick={() => fileInputRef.current?.click()}
          style={{
            border: `2px dashed ${dragOver ? '#7dd3fc' : '#334155'}`,
            borderRadius: 12,
            padding: '40px 24px',
            textAlign: 'center',
            cursor: 'pointer',
            background: dragOver ? '#7dd3fc0a' : '#1e293b',
            transition: 'border-color 0.15s, background 0.15s',
          }}
        >
          <div style={{ fontSize: 32, marginBottom: 12 }}>↓</div>
          <div style={{ color: '#e2e8f0', fontSize: 14, fontWeight: 600, marginBottom: 6 }}>
            Drop your JSON file here
          </div>
          <div style={{ color: '#64748b', fontSize: 12 }}>
            or click to browse — BioSense · Oura · Garmin · Apple Health
          </div>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            onChange={onFileChange}
            style={{ display: 'none' }}
          />
        </div>

        {message && (
          <div style={{
            marginTop: 12,
            padding: '10px 14px',
            borderRadius: 8,
            background: '#1e293b',
            border: `1px solid ${statusColor}`,
            color: statusColor,
            fontSize: 13,
          }}>
            {status === 'uploading' ? 'Uploading…' : message}
          </div>
        )}

        <div style={{ marginTop: 16, fontSize: 12, color: '#475569', lineHeight: 1.7 }}>
          <strong style={{ color: '#64748b' }}>Expected format:</strong>
          <pre style={{
            marginTop: 8,
            background: '#0f172a',
            border: '1px solid #1e293b',
            borderRadius: 8,
            padding: '10px 14px',
            color: '#94a3b8',
            fontSize: 11,
            overflowX: 'auto',
          }}>
{`{
  "source": "biosense",
  "samples": [
    {
      "recorded_at": "2026-04-01T08:00:00Z",
      "chi_ze_eeg": 0.72,
      "chi_ze_hrv": 0.68
    }
  ]
}`}
          </pre>
        </div>
      </section>

      {/* GDPR export section */}
      <section>
        <div style={{ color: '#94a3b8', fontSize: 13, marginBottom: 16, fontWeight: 600 }}>
          DATA EXPORT (GDPR)
        </div>
        <a
          href="/api/data/export"
          style={{
            display: 'inline-block',
            padding: '10px 20px',
            background: '#1e293b',
            border: '1px solid #334155',
            borderRadius: 8,
            color: '#e2e8f0',
            fontSize: 13,
            textDecoration: 'none',
            fontWeight: 600,
          }}
        >
          Download my data (JSON)
        </a>
        <div style={{ marginTop: 8, fontSize: 12, color: '#475569' }}>
          Includes all Ze samples, posts, and study enrollments.
        </div>
      </section>
    </div>
  )
}
