import { createContext, useState, useRef, useEffect, type ReactNode } from 'react'
import { useNavigate } from 'react-router-dom'
import { initApiClient } from '../api/client'
import { getMeApi, logoutApi } from '../api/authApi'
import type { User } from '../types/user'

// User 型の再エクスポート（useAuth.ts 等の既存 import との後方互換性維持）
export type { User } from '../types/user'

type AuthContextType = {
  isLoggedIn: boolean
  isInitializing: boolean
  user: User | null
  login: (user: User, accessToken: string) => void
  logout: () => Promise<void>
  getAccessToken: () => string | null
}

export const AuthContext = createContext<AuthContextType | null>(null)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [isInitializing, setIsInitializing] = useState(true)
  // AT はメモリのみで管理（再レンダリング不要のため useRef）（AC-4）
  const accessTokenRef = useRef<string | null>(null)
  const navigate = useNavigate()

  const getAccessToken = (): string | null => accessTokenRef.current

  const setAccessToken = (token: string): void => {
    accessTokenRef.current = token
  }

  // 認証失敗時のコールバック（AT リフレッシュ失敗時）（AC-18）
  const handleAuthFailure = (): void => {
    accessTokenRef.current = null
    setUser(null)
    navigate('/login')
  }

  // ログイン（AC-2）
  const login = (newUser: User, accessToken: string): void => {
    accessTokenRef.current = accessToken
    setUser(newUser)
  }

  // ログアウト（AC-3）: POST /api/v1/auth/logout と連携、成否によらずクライアント状態クリア
  // バックエンドは CookieJar から RT を取得するため AT ヘッダー不要
  const logout = async (): Promise<void> => {
    await logoutApi().catch(() => {
      // RT は httpOnly Cookie のためフロントから直接削除不可。サーバーエラー時もクライアント状態をクリアする
    })
    accessTokenRef.current = null
    setUser(null)
  }

  // isLoggedIn は user の存在で判定（不整合防止）
  const isLoggedIn = user !== null

  useEffect(() => {
    // initApiClient を登録（AT 取得・更新・失敗コールバック）（AC-16）
    initApiClient(getAccessToken, setAccessToken, handleAuthFailure)

    // マウント時に /auth/me を呼んで認証状態を復元（AC-13）
    // 順序: /auth/refresh → /auth/me
    // 初回マウント時は AT がメモリにないため、先に RT で AT を取得してから me を呼ぶ必要がある
    const restoreSession = async (): Promise<void> => {
      // まず RT を使って AT を取得する
      // 初期化時のセッション復元（client.ts の apiFetch 経由の 401 インターセプトとは別経路）
      try {
        const refreshRes = await fetch('/api/v1/auth/refresh', {
          method: 'POST',
          credentials: 'include',
        })

        if (!refreshRes.ok) {
          // RT 無効 → 未ログイン確定（AC-15）
          setIsInitializing(false)
          return
        }

        const refreshData = (await refreshRes.json()) as { access_token: string }
        const newToken = refreshData.access_token
        accessTokenRef.current = newToken

        // AT を使って me エンドポイントを呼ぶ（AC-13）
        const me = await getMeApi(newToken)
        setUser(me)
      } catch {
        // エラー時は未ログイン扱い
      } finally {
        setIsInitializing(false)
      }
    }

    restoreSession()
    // eslint-disable-next-line react-hooks/exhaustive-deps
    // navigate は react-router が安定参照を保証するため依存配列から除外
  }, [])

  return (
    <AuthContext.Provider
      value={{ isLoggedIn, isInitializing, user, login, logout, getAccessToken }}
    >
      {children}
    </AuthContext.Provider>
  )
}
