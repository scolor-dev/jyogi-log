import { Navigate, Outlet } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'

export default function PrivateRoute() {
  const { isLoggedIn, isInitializing } = useAuth()

  // 初期化中はリダイレクトしない（AC-14）
  if (isInitializing) {
    return null
  }

  if (!isLoggedIn) {
    return <Navigate to="/login" replace />
  }

  return <Outlet />
}
