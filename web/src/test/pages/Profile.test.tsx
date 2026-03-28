import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { AuthContext, type User } from '../../stores/authContext'
import Profile from '../../pages/Profile'

const MOCK_USER: User = {
  uuid: 'test-uuid-1234',
  identifier: 'test@example.com',
  display_name: 'テストユーザー',
}

function renderWithAuth(user: User | null) {
  return render(
    <AuthContext.Provider
      value={{
        isLoggedIn: user !== null,
        isInitializing: false,
        user,
        login: vi.fn(),
        logout: vi.fn(),
        getAccessToken: vi.fn().mockReturnValue(null),
      }}
    >
      <Profile />
    </AuthContext.Provider>
  )
}

describe('Profile', () => {
  // AC-28: user が null → null を返す（何もレンダリングしない）
  it('AC-28: user が null のとき何もレンダリングしない', () => {
    const { container } = renderWithAuth(null)
    expect(container.firstChild).toBeNull()
  })

  describe('表示モード', () => {
    // AC-18: display_name がアバター見出し + ProfileRow の2箇所に表示される
    it('AC-18: display_name がアバター見出しと ProfileRow の2箇所に表示される', () => {
      renderWithAuth(MOCK_USER)
      expect(screen.getAllByText('テストユーザー')).toHaveLength(2)
    })

    // AC-19/20: identifier（メールアドレス）が表示される
    it('AC-19/20: identifier（メールアドレス）が表示される', () => {
      renderWithAuth(MOCK_USER)
      // アバター下のサブテキストと ProfileRow の2箇所に表示される
      const emailElements = screen.getAllByText('test@example.com')
      expect(emailElements.length).toBeGreaterThanOrEqual(1)
    })

    // AC-21: 表示モードで input 要素が存在しない（編集機能が削除されたため）
    it('AC-21: 表示モードで input 要素が存在しない', () => {
      renderWithAuth(MOCK_USER)
      expect(screen.queryByRole('textbox')).toBeNull()
    })
  })
})
