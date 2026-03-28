import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { AuthContext, type User } from '../../stores/authContext'
import Login from '../../pages/Login'
import * as authApi from '../../api/authApi'

const MOCK_USER: User = {
  uuid: 'test-uuid-1234',
  identifier: 'test@example.com',
  display_name: 'テストユーザー',
}

const MOCK_TOKEN = 'mock-access-token'

function renderLogin(loginFn = vi.fn()) {
  return render(
    <AuthContext.Provider
      value={{
        isLoggedIn: false,
        isInitializing: false,
        user: null,
        login: loginFn,
        logout: vi.fn(),
        getAccessToken: vi.fn().mockReturnValue(null),
      }}
    >
      <MemoryRouter initialEntries={['/login']}>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/dashboard" element={<div>ダッシュボード</div>} />
        </Routes>
      </MemoryRouter>
    </AuthContext.Provider>
  )
}

beforeEach(() => {
  vi.spyOn(authApi, 'loginApi').mockResolvedValue({ user: MOCK_USER, access_token: MOCK_TOKEN })
})

afterEach(() => {
  vi.restoreAllMocks()
})

describe('Login', () => {
  // AC-5: email フィールドが存在する
  it('AC-5: email フィールドが存在する', () => {
    renderLogin()
    expect(screen.getByLabelText('メールアドレス')).toBeInTheDocument()
  })

  // AC-6/7: フォーム送信後に loginApi が呼ばれ、成功時に /dashboard へ遷移する
  it('AC-6/7: フォーム送信後に loginApi が呼ばれ、成功時に /dashboard へ遷移する', async () => {
    const user = userEvent.setup()
    const loginFn = vi.fn()
    renderLogin(loginFn)

    await user.type(screen.getByLabelText('メールアドレス'), 'test@example.com')
    // パスワードは type="password" のため role="textbox" では取得できないので getByLabelText を使用
    await user.type(screen.getByLabelText('パスワード'), 'password123')
    await user.click(screen.getByRole('button', { name: 'ログイン' }))

    await waitFor(() => {
      expect(authApi.loginApi).toHaveBeenCalledWith('test@example.com', 'password123')
    })

    await waitFor(() => {
      expect(loginFn).toHaveBeenCalledWith(MOCK_USER, MOCK_TOKEN)
    })

    await waitFor(() => {
      expect(screen.getByText('ダッシュボード')).toBeInTheDocument()
    })
  })

  // AC-8: loginApi が ApiError を throw した場合にエラーメッセージが表示される
  it('AC-8: loginApi が ApiError を throw した場合にエラーメッセージが表示される', async () => {
    vi.spyOn(authApi, 'loginApi').mockRejectedValue(
      new authApi.ApiError(401, 'メールアドレスまたはパスワードが正しくありません')
    )

    const user = userEvent.setup()
    renderLogin()

    await user.type(screen.getByLabelText('メールアドレス'), 'wrong@example.com')
    await user.type(screen.getByLabelText('パスワード'), 'wrongpassword')
    await user.click(screen.getByRole('button', { name: 'ログイン' }))

    await waitFor(() => {
      expect(screen.getByText('メールアドレスまたはパスワードが正しくありません')).toBeInTheDocument()
    })
  })
})
