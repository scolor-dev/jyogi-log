import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { AuthContext, type User } from '../../stores/authContext'
import PrivateRoute from '../../components/PrivateRoute'

const MOCK_USER: User = {
  displayName: 'テストユーザー',
  bio: 'テスト用の自己紹介文',
  department: '情報工学科',
  grade: '3年',
}

function renderPrivateRoute(isLoggedIn: boolean) {
  return render(
    <AuthContext.Provider
      value={{
        isLoggedIn,
        user: isLoggedIn ? MOCK_USER : null,
        login: vi.fn(),
        logout: vi.fn(),
        updateUser: vi.fn(),
      }}
    >
      <MemoryRouter initialEntries={['/protected']}>
        <Routes>
          <Route element={<PrivateRoute />}>
            <Route path="/protected" element={<div>保護されたコンテンツ</div>} />
          </Route>
          <Route path="/login" element={<div>ログインページ</div>} />
        </Routes>
      </MemoryRouter>
    </AuthContext.Provider>
  )
}

describe('PrivateRoute', () => {
  // AC-29: isLoggedIn が false → /login へリダイレクト
  it('AC-29: 未ログイン時は /login へリダイレクトしてログインページを表示する', () => {
    renderPrivateRoute(false)
    expect(screen.getByText('ログインページ')).toBeInTheDocument()
    expect(screen.queryByText('保護されたコンテンツ')).toBeNull()
  })

  // AC-30: isLoggedIn が true → 子ルートのコンテンツが表示される
  it('AC-30: ログイン済みのとき子ルートのコンテンツが表示される', () => {
    renderPrivateRoute(true)
    expect(screen.getByText('保護されたコンテンツ')).toBeInTheDocument()
    expect(screen.queryByText('ログインページ')).toBeNull()
  })
})
