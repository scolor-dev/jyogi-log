import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { AuthContext, type User } from '../../stores/authContext'
import Signup from '../../pages/Signup'
import * as authApi from '../../api/authApi'

const MOCK_USER: User = {
  uuid: 'test-uuid-1234',
  identifier: 'newuser@example.com',
  display_name: '新しいユーザー',
}

const MOCK_TOKEN = 'mock-access-token'

function renderSignup(loginFn = vi.fn()) {
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
      <MemoryRouter initialEntries={['/signup']}>
        <Routes>
          <Route path="/signup" element={<Signup />} />
          <Route path="/dashboard" element={<div>ダッシュボード</div>} />
        </Routes>
      </MemoryRouter>
    </AuthContext.Provider>
  )
}

beforeEach(() => {
  vi.spyOn(authApi, 'signupApi').mockResolvedValue({ user: MOCK_USER, access_token: MOCK_TOKEN })
})

afterEach(() => {
  vi.restoreAllMocks()
})

describe('Signup', () => {
  // AC-9: email / 表示名 / パスワード フィールドが存在する
  it('AC-9: email / 表示名 / パスワード フィールドが存在する', () => {
    renderSignup()
    expect(screen.getByLabelText('メールアドレス')).toBeInTheDocument()
    expect(screen.getByLabelText('表示名')).toBeInTheDocument()
    expect(screen.getByLabelText('パスワード')).toBeInTheDocument()
  })

  // AC-10/11: フォーム送信後に signupApi が呼ばれ、成功時に /dashboard へ遷移する
  it('AC-10/11: フォーム送信後に signupApi が呼ばれ、成功時に /dashboard へ遷移する', async () => {
    const user = userEvent.setup()
    const loginFn = vi.fn()
    renderSignup(loginFn)

    await user.type(screen.getByLabelText('メールアドレス'), 'newuser@example.com')
    await user.type(screen.getByLabelText('表示名'), '新しいユーザー')
    await user.type(screen.getByLabelText('パスワード'), 'password123')
    await user.click(screen.getByRole('button', { name: '登録' }))

    await waitFor(() => {
      expect(authApi.signupApi).toHaveBeenCalledWith(
        'newuser@example.com',
        '新しいユーザー',
        'password123'
      )
    })

    await waitFor(() => {
      expect(loginFn).toHaveBeenCalledWith(MOCK_USER, MOCK_TOKEN)
    })

    await waitFor(() => {
      expect(screen.getByText('ダッシュボード')).toBeInTheDocument()
    })
  })

  // AC-12a: signupApi が 409 ApiError を throw した場合にメールアドレス重複メッセージが表示される
  it('AC-12a: signupApi が 409 ApiError を throw した場合にメールアドレス重複メッセージが表示される', async () => {
    vi.spyOn(authApi, 'signupApi').mockRejectedValue(
      new authApi.ApiError(409, 'Conflict')
    )

    const user = userEvent.setup()
    renderSignup()

    await user.type(screen.getByLabelText('メールアドレス'), 'existing@example.com')
    await user.type(screen.getByLabelText('表示名'), '既存ユーザー')
    await user.type(screen.getByLabelText('パスワード'), 'password123')
    await user.click(screen.getByRole('button', { name: '登録' }))

    await waitFor(() => {
      expect(screen.getByText('このメールアドレスはすでに登録されています')).toBeInTheDocument()
    })
  })

  // AC-12b: signupApi が 400 ApiError を throw した場合にバリデーションエラーメッセージが表示される
  it('AC-12b: signupApi が 400 ApiError を throw した場合にバリデーションエラーメッセージが表示される', async () => {
    vi.spyOn(authApi, 'signupApi').mockRejectedValue(
      new authApi.ApiError(400, 'パスワードは8文字以上で入力してください')
    )

    const user = userEvent.setup()
    renderSignup()

    await user.type(screen.getByLabelText('メールアドレス'), 'user@example.com')
    await user.type(screen.getByLabelText('表示名'), 'ユーザー')
    await user.type(screen.getByLabelText('パスワード'), '123')
    await user.click(screen.getByRole('button', { name: '登録' }))

    await waitFor(() => {
      expect(screen.getByText('パスワードは8文字以上で入力してください')).toBeInTheDocument()
    })
  })
})
