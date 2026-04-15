import React from 'react'
import { useQuery, useQueryClient } from '@tanstack/react-query'
import { PostCard } from '../components/feed/PostCard'
import { CreatePost } from '../components/feed/CreatePost'
import api from '../hooks/useApi'
import { useAuthStore } from '../store'
import type { Post } from '../types'

export function Feed() {
  const isAuth = useAuthStore(s => s.isAuthenticated())
  const qc = useQueryClient()

  const { data: posts, isLoading } = useQuery<Post[]>({
    queryKey: ['feed'],
    queryFn: async () => {
      const { data } = await api.get('/feed')
      return data
    },
    refetchInterval: 30_000,
  })

  return (
    <div style={{ maxWidth: 680, margin: '0 auto', padding: 16 }}>
      {isAuth && (
        <CreatePost onCreated={() => qc.invalidateQueries({ queryKey: ['feed'] })} />
      )}

      {isLoading ? (
        <div style={{ color: '#64748b', textAlign: 'center', padding: 40 }}>Loading feed…</div>
      ) : posts?.length === 0 ? (
        <div style={{ color: '#64748b', textAlign: 'center', padding: 40 }}>
          No posts yet. Be the first to share a Ze log.
        </div>
      ) : (
        posts?.map(post => (
          <PostCard
            key={post.id}
            post={post}
            onReacted={() => qc.invalidateQueries({ queryKey: ['feed'] })}
          />
        ))
      )}
    </div>
  )
}
