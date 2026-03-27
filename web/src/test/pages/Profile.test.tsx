import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, it, expect, vi } from 'vitest'
import { AuthContext, type User } from '../../stores/authContext'
import Profile from '../../pages/Profile'

const MOCK_USER: User = {
  displayName: 'テストユーザー',
  bio: 'テスト用の自己紹介文',
  department: '情報工学科',
  grade: '3年',
}

function renderWithAuth(user: User | null, updateUser = vi.fn()) {
  return render(
    <AuthContext.Provider
      value={{
        isLoggedIn: user !== null,
        user,
        login: vi.fn(),
        logout: vi.fn(),
        updateUser,
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

  describe('表示モード（isEditing = false）', () => {
    // AC-18: 表示モードで user.displayName が DOM に存在する
    it('AC-18: user.displayName が DOM に表示される', () => {
      renderWithAuth(MOCK_USER)
      expect(screen.getAllByText('テストユーザー').length).toBeGreaterThan(0)
    })

    // AC-19: 表示モードで user.bio が DOM に存在する
    it('AC-19: user.bio が DOM に表示される', () => {
      renderWithAuth(MOCK_USER)
      expect(screen.getByText('テスト用の自己紹介文')).toBeInTheDocument()
    })

    // AC-20: 表示モードで user.department と user.grade が DOM に存在する
    it('AC-20: user.department と user.grade が DOM に表示される', () => {
      renderWithAuth(MOCK_USER)
      expect(screen.getAllByText('情報工学科').length).toBeGreaterThan(0)
      expect(screen.getAllByText('3年').length).toBeGreaterThan(0)
    })

    // AC-21: 表示モードで input 要素が DOM に存在しない
    it('AC-21: input 要素が存在しない', () => {
      renderWithAuth(MOCK_USER)
      expect(screen.queryByRole('textbox')).toBeNull()
    })
  })

  describe('「編集」ボタンクリック後', () => {
    // AC-22: 「編集」ボタンクリック後 → input 要素が表示される
    it('AC-22: input 要素が表示される', async () => {
      renderWithAuth(MOCK_USER)
      await userEvent.click(screen.getByRole('button', { name: '編集' }))
      expect(screen.getAllByRole('textbox').length).toBeGreaterThan(0)
    })

    // AC-23: 「編集」ボタンクリック後 → 「編集」ボタンが消える
    it('AC-23: 「編集」ボタンが消える', async () => {
      renderWithAuth(MOCK_USER)
      await userEvent.click(screen.getByRole('button', { name: '編集' }))
      expect(screen.queryByRole('button', { name: '編集' })).toBeNull()
    })
  })

  describe('「保存」ボタンクリック後', () => {
    // AC-24: 「保存」ボタンクリック後 → updateUser が1回呼ばれる
    it('AC-24: updateUser が1回呼ばれる', async () => {
      const updateUser = vi.fn()
      renderWithAuth(MOCK_USER, updateUser)
      await userEvent.click(screen.getByRole('button', { name: '編集' }))
      await userEvent.click(screen.getByRole('button', { name: '保存' }))
      expect(updateUser).toHaveBeenCalledTimes(1)
    })

    // AC-25: 「保存」ボタンクリック後 → 表示モードに戻る（「編集」ボタンが再表示）
    it('AC-25: 表示モードに戻り「編集」ボタンが再表示される', async () => {
      renderWithAuth(MOCK_USER)
      await userEvent.click(screen.getByRole('button', { name: '編集' }))
      await userEvent.click(screen.getByRole('button', { name: '保存' }))
      expect(screen.getByRole('button', { name: '編集' })).toBeInTheDocument()
    })
  })

  describe('「キャンセル」ボタンクリック後', () => {
    // AC-26: 「キャンセル」ボタンクリック後 → 表示モードに戻る
    it('AC-26: 表示モードに戻る', async () => {
      renderWithAuth(MOCK_USER)
      await userEvent.click(screen.getByRole('button', { name: '編集' }))
      await userEvent.click(screen.getByRole('button', { name: 'キャンセル' }))
      expect(screen.getByRole('button', { name: '編集' })).toBeInTheDocument()
    })

    // AC-27: 「キャンセル」ボタンクリック後 → updateUser は呼ばれない
    it('AC-27: updateUser は呼ばれない', async () => {
      const updateUser = vi.fn()
      renderWithAuth(MOCK_USER, updateUser)
      await userEvent.click(screen.getByRole('button', { name: '編集' }))
      await userEvent.click(screen.getByRole('button', { name: 'キャンセル' }))
      expect(updateUser).not.toHaveBeenCalled()
    })
  })
})
