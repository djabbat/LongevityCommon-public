import { useEffect } from 'react'
import { useQuery, useQueryClient } from '@tanstack/react-query'
import { PostCard } from '../components/feed/PostCard'
import { CreatePost } from '../components/feed/CreatePost'
import api from '../hooks/useApi'
import { useRealtime } from '../hooks/useRealtime'
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
    // Realtime channel pushes via `feed:lobby`. Polling is the fallback
    // for unauthenticated readers who don't open a websocket; bumped to
    // 60s once realtime is wired (was 30s).
    refetchInterval: isAuth ? 60_000 : 30_000,
  })

  // Phase 4.5 (2026-05-07): live feed via Phoenix Channel `feed:lobby`.
  // Server publishes `new_post` / `post_updated` / `post_deleted` events;
  // client invalidates the feed query (cheaper than maintaining
  // client-side ordering).
  const realtime = useRealtime('feed:lobby', { autoJoin: isAuth })
  useEffect(() => {
    if (!isAuth) return
    realtime.on('new_post',     () => qc.invalidateQueries({ queryKey: ['feed'] }))
    realtime.on('post_updated', () => qc.invalidateQueries({ queryKey: ['feed'] }))
    realtime.on('post_deleted', () => qc.invalidateQueries({ queryKey: ['feed'] }))
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAuth])

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
