import React from 'react'
import type { Post, PostType } from '../../types'
import { formatDistanceToNow, parseISO } from 'date-fns'
import api from '../../hooks/useApi'
import { useAuthStore } from '../../store'

const TYPE_LABELS: Record<PostType, string> = {
  ze_log: 'Ze Log',
  science_thread: 'Science',
  study_invite: 'Study',
  debate: 'Debate',
}

const TYPE_COLORS: Record<PostType, string> = {
  ze_log: '#7dd3fc',
  science_thread: '#a78bfa',
  study_invite: '#34d399',
  debate: '#fb923c',
}

interface Props {
  post: Post
  onReacted?: () => void
}

export function PostCard({ post, onReacted }: Props) {
  const isAuth = useAuthStore((s) => s.isAuthenticated())

  const react = async (type: string) => {
    if (!isAuth) return
    await api.post(`/posts/${post.id}/react`, { type })
    onReacted?.()
  }

  return (
    <div style={{
      background: '#1e293b',
      border: '1px solid #334155',
      borderRadius: 10,
      padding: 16,
      marginBottom: 12,
    }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
        <span style={{
          fontSize: 11,
          fontWeight: 700,
          padding: '2px 8px',
          borderRadius: 99,
          background: TYPE_COLORS[post.type] + '22',
          color: TYPE_COLORS[post.type],
          fontFamily: 'monospace',
        }}>
          {TYPE_LABELS[post.type]}
        </span>
        <span style={{ fontSize: 13, fontWeight: 600, color: '#7dd3fc' }}>
          @{post.author_username}
        </span>
        {post.author_degree_verified && (
          <span title="Verified degree" style={{ fontSize: 12 }}>🎓</span>
        )}
        <span style={{ fontSize: 11, color: '#64748b', marginLeft: 'auto' }}>
          {formatDistanceToNow(parseISO(post.created_at), { addSuffix: true })}
        </span>
      </div>

      <div style={{ fontSize: 14, lineHeight: 1.6, color: '#cbd5e1', marginBottom: 10, whiteSpace: 'pre-wrap' }}>
        {post.content}
      </div>

      {post.doi && (
        <div style={{ fontSize: 12, fontFamily: 'monospace', marginBottom: 8 }}>
          <span style={{ color: '#94a3b8' }}>DOI: </span>
          <span style={{ color: post.doi_verified ? '#22c55e' : '#f87171' }}>
            {post.doi} {post.doi_verified ? '✓' : '✗ unverified'}
          </span>
        </div>
      )}

      {(post.code_url || post.data_url) && (
        <div style={{ fontSize: 12, marginBottom: 8, display: 'flex', gap: 12 }}>
          {post.code_url && <a href={post.code_url} target="_blank" rel="noreferrer" style={{ color: '#7dd3fc' }}>📎 Code</a>}
          {post.data_url && <a href={post.data_url} target="_blank" rel="noreferrer" style={{ color: '#7dd3fc' }}>📊 Data</a>}
        </div>
      )}

      <div style={{ display: 'flex', gap: 12, fontSize: 13 }}>
        <ReactionBtn icon="👍" count={post.reactions.support} onClick={() => react('support')} />
        <ReactionBtn icon="🔁" count={post.reactions.replicate} onClick={() => react('replicate')} label="Replicate" />
        <ReactionBtn icon="🔗" count={post.reactions.cite} onClick={() => react('cite')} label="Cite" />
        <ReactionBtn icon="⚠️" count={post.reactions.challenge} onClick={() => react('challenge')} label="Challenge" />
      </div>
    </div>
  )
}

function ReactionBtn({ icon, count, onClick, label }: {
  icon: string; count: number; onClick: () => void; label?: string
}) {
  return (
    <button
      onClick={onClick}
      style={{
        background: 'none',
        border: '1px solid #334155',
        borderRadius: 6,
        padding: '2px 8px',
        cursor: 'pointer',
        color: '#94a3b8',
        fontSize: 13,
        display: 'flex',
        alignItems: 'center',
        gap: 4,
      }}
    >
      {icon} {count > 0 && count}
    </button>
  )
}
