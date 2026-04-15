import { useQuery } from '@tanstack/react-query'
import api from './useApi'
import type { ZeProfile, ZeTrend } from '../types'

export function useZeProfile() {
  return useQuery<ZeProfile>({
    queryKey: ['dashboard'],
    queryFn: async () => {
      const { data } = await api.get('/dashboard')
      return data
    },
    refetchInterval: 60_000, // refresh every minute
  })
}

export function useZeTrend(period: 7 | 30 | 365 = 30) {
  return useQuery<ZeTrend>({
    queryKey: ['trend', period],
    queryFn: async () => {
      const { data } = await api.get(`/dashboard/trend?period=${period}`)
      return data
    },
  })
}
