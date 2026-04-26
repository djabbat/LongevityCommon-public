import React from 'react'
import { BrowserRouter, Routes, Route, NavLink, Navigate } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useAuthStore } from './store'
import { Feed } from './pages/Feed'
import { Dashboard } from './pages/Dashboard'
import { Studies } from './pages/Studies'
import { Login } from './pages/Login'
import { Settings } from './pages/Settings'
import { Profile } from './pages/Profile'

const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: 1, staleTime: 30_000 } },
})

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const isAuth = useAuthStore(s => s.isAuthenticated())
  return isAuth ? <>{children}</> : <Navigate to="/login" replace />
}

function Layout({ children }: { children: React.ReactNode }) {
  const { user, clearAuth } = useAuthStore()

  const navLink: React.CSSProperties = {
    color: '#94a3b8',
    textDecoration: 'none',
    fontSize: 14,
    padding: '4px 0',
  }
  const activeStyle: React.CSSProperties = {
    color: '#7dd3fc',
    fontWeight: 700,
    borderBottom: '2px solid #7dd3fc',
  }

  return (
    <div style={{ minHeight: '100vh', background: '#0f172a', color: '#e2e8f0' }}>
      <nav style={{
        display: 'flex',
        alignItems: 'center',
        gap: 24,
        padding: '12px 24px',
        borderBottom: '1px solid #1e293b',
        position: 'sticky',
        top: 0,
        background: '#0f172a',
        zIndex: 100,
      }}>
        <NavLink to="/" style={{ ...navLink, fontWeight: 800, fontSize: 16, color: '#7dd3fc', fontFamily: 'monospace' }}>
          LongevityCommon
        </NavLink>

        <NavLink to="/" style={({ isActive }) => ({ ...navLink, ...(isActive ? activeStyle : {}) })}>
          Feed
        </NavLink>
        {user && (
          <NavLink to="/dashboard" style={({ isActive }) => ({ ...navLink, ...(isActive ? activeStyle : {}) })}>
            Dashboard
          </NavLink>
        )}
        <NavLink to="/lab" style={({ isActive }) => ({ ...navLink, ...(isActive ? activeStyle : {}) })}>
          Lab
        </NavLink>

        <div style={{ marginLeft: 'auto', display: 'flex', alignItems: 'center', gap: 16 }}>
          {user ? (
            <>
              <NavLink
                to={`/u/${user.username}`}
                style={{ fontSize: 13, color: '#64748b', textDecoration: 'none' }}
              >
                @{user.username}
              </NavLink>
              <NavLink to="/settings" style={({ isActive }) => ({ ...navLink, ...(isActive ? activeStyle : {}) })}>
                Settings
              </NavLink>
              <button
                onClick={clearAuth}
                style={{ background: 'none', border: 'none', color: '#64748b', cursor: 'pointer', fontSize: 13 }}
              >
                Logout
              </button>
            </>
          ) : (
            <NavLink to="/login" style={{ ...navLink, color: '#7dd3fc' }}>Login</NavLink>
          )}
        </div>
      </nav>

      <main style={{ padding: '24px 16px' }}>
        {children}
      </main>
    </div>
  )
}

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/" element={<Layout><Feed /></Layout>} />
          <Route path="/dashboard" element={
            <Layout>
              <ProtectedRoute><Dashboard /></ProtectedRoute>
            </Layout>
          } />
          <Route path="/lab" element={<Layout><Studies /></Layout>} />
          <Route path="/u/:username" element={<Layout><Profile /></Layout>} />
          <Route path="/settings" element={
            <Layout>
              <ProtectedRoute><Settings /></ProtectedRoute>
            </Layout>
          } />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  )
}
