/**
 * useRealtime — Phoenix Channel client (Phase 4.5, 2026-05-07).
 *
 * Wraps phoenix-js to subscribe to realtime channels exposed by
 * `realtime/lib/longevitycommon_web/channels/`:
 *   - `feed:lobby`       — live post stream
 *   - `study:<study_id>` — per-study live updates
 *   - `ze_clock:<user>`  — biological-clock streaming
 *
 * Auth: passes JWT (from auth store) as `token` URL param —
 * `realtime/lib/longevitycommon_web/user_socket.ex` validates it.
 */
import { useEffect, useRef, useState } from 'react'
import { Socket, Channel, Push } from 'phoenix'
import { useAuthStore } from '../store'

const REALTIME_URL =
    (import.meta as any).env?.VITE_REALTIME_URL ||
    'wss://app.longevity.ge/realtime/socket'

interface UseRealtimeOptions {
    /** Auto-join on mount; default true. */
    autoJoin?: boolean
    /** Initial channel params (e.g. last-seen cursor for replay). */
    params?: Record<string, unknown>
}

interface UseRealtimeReturn {
    channel: Channel | null
    state: 'idle' | 'joining' | 'joined' | 'errored' | 'closed'
    on: (event: string, cb: (payload: any) => void) => void
    push: (event: string, payload: unknown) => Push | null
}

let sharedSocket: Socket | null = null

function getSocket(token: string | null): Socket {
    if (sharedSocket && sharedSocket.isConnected()) return sharedSocket
    sharedSocket = new Socket(REALTIME_URL, {
        params: token ? { token } : {},
        reconnectAfterMs: (tries: number) =>
            [1000, 2000, 5000, 10_000][Math.min(tries, 3)],
    })
    sharedSocket.connect()
    return sharedSocket
}

export function useRealtime(
    topic: string,
    options: UseRealtimeOptions = {},
): UseRealtimeReturn {
    const { autoJoin = true, params = {} } = options
    const token = useAuthStore((s) => s.token)
    const [state, setState] = useState<UseRealtimeReturn['state']>('idle')
    const channelRef = useRef<Channel | null>(null)
    const handlersRef = useRef<Array<{ event: string; cb: (p: any) => void }>>([])

    useEffect(() => {
        if (!autoJoin) return
        const socket = getSocket(token)
        const ch = socket.channel(topic, params)
        channelRef.current = ch

        // Re-attach any handlers registered via on() before join completes.
        handlersRef.current.forEach(({ event, cb }) => ch.on(event, cb))

        setState('joining')
        ch.join()
            .receive('ok', () => setState('joined'))
            .receive('error', () => setState('errored'))
            .receive('timeout', () => setState('errored'))

        return () => {
            ch.leave()
            channelRef.current = null
            setState('closed')
        }
    }, [topic, autoJoin, token])

    const on = (event: string, cb: (payload: any) => void) => {
        handlersRef.current.push({ event, cb })
        if (channelRef.current) {
            channelRef.current.on(event, cb)
        }
    }

    const push = (event: string, payload: unknown): Push | null =>
        channelRef.current
            ? channelRef.current.push(event, payload as object)
            : null

    return { channel: channelRef.current, state, on, push }
}
