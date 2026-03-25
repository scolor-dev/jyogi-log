import { createContext, useState, type ReactNode } from 'react'

// ユーザープロフィールの型
export type User = {
  displayName: string
  bio: string
  department: string
  grade: string
}

// モックユーザーデータ（後でAPIから取得するデータに差し替える）
const MOCK_USER: User = {
  displayName: 'Jyogi User',
  bio: 'Building OAuth from scratch.',
  department: '情報工学科',
  grade: '3年',
}

type AuthContextType = {
  isLoggedIn: boolean
  user: User | null
  login: () => void
  logout: () => void
  updateUser: (newUser: User) => void
}

export const AuthContext = createContext<AuthContextType | null>(null)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(
    () => (localStorage.getItem('token') ? MOCK_USER : null)
  )

  // user が存在するかどうかでログイン状態を判定する（別 state にすると不整合が起きうる）
  const isLoggedIn = user !== null

  const login = () => {
    localStorage.setItem('token', 'dummy')
    setUser(MOCK_USER)
  }

  const logout = () => {
    localStorage.removeItem('token')
    setUser(null)
  }

  // プロフィールを更新する（保存ボタンを押したときに呼ぶ）
  const updateUser = (newUser: User) => {
    setUser(newUser)
  }

  return (
    <AuthContext.Provider value={{ isLoggedIn, user, login, logout, updateUser }}>
      {children}
    </AuthContext.Provider>
  )
}
