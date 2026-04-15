import React from 'react'
import { useParams } from 'react-router-dom'
import { useQuery } from '@tanstack/react-query'
import api from '../hooks/useApi'
import { ZeProfileCard } from '../components/ui/ZeProfileCard'
import type { ZeProfile } from '../types'

interface PublicUserResponse {
  id: string
  username: string
  degree_verified: boolean
  fclc_node_active: boolean
  country_code: string | null
  created_at: string
  ze_profile: ZeProfile | null
}

export function Profile() {
  const { username } = useParams<{ username: string }>()

  const { data, isLoading, isError } = useQuery<PublicUserResponse>({
    queryKey: ['profile', username],
    queryFn: async () => {
      const { data } = await api.get<PublicUserResponse>(`/users/by-username/${username}`)
      return data
    },
    enabled: !!username,
    retry: 1,
  })

  if (isLoading) {
    return (
      <div style={{ color: '#64748b', textAlign: 'center', padding: 60 }}>
        Loading @{username}…
      </div>
    )
  }

  if (isError || !data) {
    return (
      <div style={{ color: '#ef4444', textAlign: 'center', padding: 60 }}>
        User @{username} not found.
      </div>
    )
  }

  return (
    <div style={{ maxWidth: 600, margin: '0 auto', padding: 16 }}>
      {/* Header */}
      <div style={{
        background: '#1e293b',
        borderRadius: 12,
        padding: 24,
        marginBottom: 20,
        border: '1px solid #334155',
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
          <div style={{
            width: 56,
            height: 56,
            borderRadius: '50%',
            background: '#0369a1',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 22,
            fontWeight: 800,
            color: '#e2e8f0',
          }}>
            {data.username[0].toUpperCase()}
          </div>
          <div>
            <div style={{ fontWeight: 700, fontSize: 18, color: '#e2e8f0' }}>
              @{data.username}
            </div>
            <div style={{ fontSize: 12, color: '#64748b', marginTop: 4 }}>
              Joined {new Date(data.created_at).toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
              {data.country_code && ` · ${data.country_code}`}
            </div>
          </div>
          <div style={{ marginLeft: 'auto', display: 'flex', flexDirection: 'column', gap: 4, alignItems: 'flex-end' }}>
            {data.degree_verified && (
              <span style={{ fontSize: 11, color: '#22c55e', background: '#14532d33', padding: '2px 8px', borderRadius: 999 }}>
                Verified Researcher
              </span>
            )}
            {data.fclc_node_active && (
              <span style={{ fontSize: 11, color: '#7dd3fc', background: '#0369a133', padding: '2px 8px', borderRadius: 999 }}>
                FCLC Node
              </span>
            )}
          </div>
        </div>
      </div>

      {/* Ze Profile */}
      {data.ze_profile ? (
        <div style={{ marginBottom: 20 }}>
          <div style={{ fontSize: 13, color: '#94a3b8', marginBottom: 12, fontWeight: 600 }}>
            ZE PROFILE
          </div>
          <ZeProfileCard profile={data.ze_profile} />
        </div>
      ) : (
        <div style={{
          background: '#1e293b',
          border: '1px dashed #334155',
          borderRadius: 12,
          padding: 32,
          textAlign: 'center',
          color: '#64748b',
          fontSize: 14,
        }}>
          No Ze data shared publicly.
        </div>
      )}
    </div>
  )
}
