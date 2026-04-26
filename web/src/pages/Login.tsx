import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import api from '../hooks/useApi'
import { useAuthStore } from '../store'

type Step = 'email' | 'otp'

export function Login() {
  const [step, setStep] = useState<Step>('email')
  const [email, setEmail] = useState('')
  const [username, setUsername] = useState('')
  const [otp, setOtp] = useState('')
  const [isRegister, setIsRegister] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [devOtp, setDevOtp] = useState<string | null>(null)
  const { setAuth } = useAuthStore()
  const navigate = useNavigate()

  const sendOtp = async () => {
    setLoading(true)
    setError(null)
    try {
      if (isRegister) {
        const { data } = await api.post('/auth/register', {
          email,
          username,
          birth_year: null,
          consent: true,
        })
        setDevOtp(data.dev_otp ?? null)
      } else {
        // For existing users, initiate login via OTP
        const { data } = await api.post('/auth/register', {
          email,
          username: username || email.split('@')[0],
          consent: true,
        })
        setDevOtp(data.dev_otp ?? null)
      }
      setStep('otp')
    } catch (e: any) {
      setError(e.response?.data || 'Something went wrong')
    } finally {
      setLoading(false)
    }
  }

  const verifyOtp = async () => {
    setLoading(true)
    setError(null)
    try {
      const { data } = await api.post('/auth/verify-otp', { email, otp })
      setAuth(data.token, data.user)
      navigate('/dashboard')
    } catch (e: any) {
      setError('Invalid or expired code')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div style={{
      minHeight: '100vh',
      background: '#0f172a',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
    }}>
      <div style={{
        background: '#1e293b',
        border: '1px solid #334155',
        borderRadius: 16,
        padding: 32,
        width: 360,
      }}>
        <div style={{ textAlign: 'center', marginBottom: 24 }}>
          <div style={{ fontSize: 24, fontWeight: 800, color: '#7dd3fc', fontFamily: 'monospace' }}>
            LongevityCommon
          </div>
          <div style={{ fontSize: 13, color: '#64748b', marginTop: 4 }}>
            Longevity social network
          </div>
        </div>

        {step === 'email' ? (
          <>
            <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
              <button
                onClick={() => setIsRegister(false)}
                style={tabStyle(!isRegister)}
              >Login</button>
              <button
                onClick={() => setIsRegister(true)}
                style={tabStyle(isRegister)}
              >Register</button>
            </div>

            {isRegister && (
              <input
                value={username}
                onChange={e => setUsername(e.target.value)}
                placeholder="Username (3–30 chars)"
                style={inputStyle}
              />
            )}

            <input
              value={email}
              onChange={e => setEmail(e.target.value)}
              placeholder="Email"
              type="email"
              style={inputStyle}
            />

            {isRegister && (
              <div style={{ fontSize: 12, color: '#64748b', marginBottom: 12 }}>
                By registering, you consent to data collection for longevity research under FCLC privacy stack (GDPR Art. 6(a)).
                You can withdraw consent and delete all data at any time.
              </div>
            )}

            <button
              onClick={sendOtp}
              disabled={loading || !email}
              style={submitBtnStyle}
            >
              {loading ? 'Sending…' : 'Send code →'}
            </button>
          </>
        ) : (
          <>
            <div style={{ fontSize: 14, color: '#94a3b8', marginBottom: 16 }}>
              Enter the 6-digit code sent to <strong>{email}</strong>
            </div>

            {devOtp && (
              <div style={{ fontSize: 13, color: '#22c55e', marginBottom: 12, fontFamily: 'monospace' }}>
                Dev OTP: {devOtp}
              </div>
            )}

            <input
              value={otp}
              onChange={e => setOtp(e.target.value)}
              placeholder="123456"
              maxLength={6}
              style={{ ...inputStyle, letterSpacing: 6, fontSize: 20, textAlign: 'center' }}
            />

            <button
              onClick={verifyOtp}
              disabled={loading || otp.length !== 6}
              style={submitBtnStyle}
            >
              {loading ? 'Verifying…' : 'Verify →'}
            </button>

            <button
              onClick={() => setStep('email')}
              style={{ background: 'none', border: 'none', color: '#64748b', cursor: 'pointer', fontSize: 13, marginTop: 8 }}
            >
              ← Back
            </button>
          </>
        )}

        {error && (
          <div style={{ color: '#f87171', fontSize: 13, marginTop: 12 }}>{error}</div>
        )}
      </div>
    </div>
  )
}

const inputStyle: React.CSSProperties = {
  width: '100%',
  background: '#0f172a',
  border: '1px solid #334155',
  borderRadius: 8,
  padding: '10px 12px',
  color: '#e2e8f0',
  fontSize: 14,
  marginBottom: 12,
  boxSizing: 'border-box',
}

const submitBtnStyle: React.CSSProperties = {
  width: '100%',
  background: '#7dd3fc',
  color: '#0f172a',
  border: 'none',
  borderRadius: 8,
  padding: '12px',
  cursor: 'pointer',
  fontWeight: 700,
  fontSize: 15,
}

const tabStyle = (active: boolean): React.CSSProperties => ({
  flex: 1,
  padding: '8px',
  border: '1px solid',
  borderColor: active ? '#7dd3fc' : '#334155',
  background: active ? '#7dd3fc22' : 'none',
  color: active ? '#7dd3fc' : '#94a3b8',
  borderRadius: 8,
  cursor: 'pointer',
  fontSize: 14,
  fontWeight: active ? 700 : 400,
  marginBottom: 12,
})
