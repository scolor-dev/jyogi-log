import { useContext } from 'react'
import { AuthContext } from '../stores/authContext'

// AuthContext を取得する便利フック
// AuthProvider の外で呼ばれた場合はエラーを出す
export function useAuth() {
  const ctx = useContext(AuthContext)
  if (!ctx) {
    throw new Error('useAuth は AuthProvider の内側で使用してください')
  }
  return ctx
}
