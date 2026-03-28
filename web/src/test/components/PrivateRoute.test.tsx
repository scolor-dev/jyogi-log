import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { AuthContext, type User } from '../../stores/authContext'
import PrivateRoute from '../../components/PrivateRoute'

const MOCK_USER: User = {
  uuid: 'test-uuid-1234',
  identifier: 'test@example.com',
  display_name: 'テストユーザー',
}

function renderPrivateRoute(isLoggedIn: boolean, isInitializing = false) {
  return render(
    <AuthContext.Provider
      value={{
        isLoggedIn,
        isInitializing,
        user: isLoggedIn ? MOCK_USER : null,
        login: vi.fn(),
        logout: vi.fn(),
        getAccessToken: vi.fn().mockReturnValue(null),
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

  // AC-14: isInitializing が true のとき何もレンダリングしない
  it('AC-14: isInitializing が true のとき何もレンダリングしない', () => {
    const { container } = renderPrivateRoute(false, true)
    // isInitializing が true のとき PrivateRoute は null を返す
    // MemoryRouter と Routes の wrapper 要素のみ残り、保護コンテンツもログインページも表示されない
    expect(screen.queryByText('保護されたコンテンツ')).toBeNull()
    expect(screen.queryByText('ログインページ')).toBeNull()
    // container 内に意味のある子要素がないことを確認
    expect(container.querySelector('div')).toBeNull()
  })
})
