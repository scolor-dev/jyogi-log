import { renderHook, act } from '@testing-library/react'
import { describe, it, expect, beforeEach } from 'vitest'
import { AuthProvider } from '../../stores/authContext'
import { useAuth } from '../../hooks/useAuth'

// AC-16: beforeEach で localStorage.clear() を実行
beforeEach(() => {
  localStorage.clear()
})

describe('useAuth / AuthContext', () => {
  // AC-17: useAuth() を AuthProvider 外で呼ぶ → エラーがスローされる
  it('AC-17: AuthProvider 外で useAuth() を呼ぶとエラーをスローする', () => {
    // AuthContext.Provider なしで useAuth を呼ぶ
    // renderHook はデフォルトで wrapper なし = AuthProvider なし
    expect(() => {
      renderHook(() => useAuth())
    }).toThrow('useAuth は AuthProvider の内側で使用してください')
  })

  describe('未ログイン状態（localStorageにトークンなし）', () => {
    // AC-6: localStorage にトークンなし → isLoggedIn が false
    it('AC-6: isLoggedIn が false である', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      expect(result.current.isLoggedIn).toBe(false)
    })

    // AC-7: localStorage にトークンなし → user が null
    it('AC-7: user が null である', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      expect(result.current.user).toBeNull()
    })
  })

  describe('login() 呼び出し後', () => {
    // AC-8: login() 呼び出し後 → isLoggedIn が true
    it('AC-8: isLoggedIn が true になる', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      act(() => {
        result.current.login()
      })
      expect(result.current.isLoggedIn).toBe(true)
    })

    // AC-9: login() 呼び出し後 → user が null でない
    it('AC-9: user が null でない', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      act(() => {
        result.current.login()
      })
      expect(result.current.user).not.toBeNull()
    })

    // AC-10: login() 呼び出し後 → localStorage に 'token' キーが存在する
    it('AC-10: localStorage に token キーが存在する', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      act(() => {
        result.current.login()
      })
      const token = localStorage.getItem('token')
      expect(token).not.toBeNull()
      expect(token).not.toBe('')
    })
  })

  describe('logout() 呼び出し後', () => {
    // ログイン済みの状態を準備するヘルパー
    function renderLoggedIn() {
      const rendered = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      act(() => {
        rendered.result.current.login()
      })
      return rendered
    }

    // AC-11: logout() 呼び出し後 → isLoggedIn が false
    it('AC-11: isLoggedIn が false になる', () => {
      const { result } = renderLoggedIn()
      act(() => {
        result.current.logout()
      })
      expect(result.current.isLoggedIn).toBe(false)
    })

    // AC-12: logout() 呼び出し後 → user が null
    it('AC-12: user が null になる', () => {
      const { result } = renderLoggedIn()
      act(() => {
        result.current.logout()
      })
      expect(result.current.user).toBeNull()
    })

    // AC-13: logout() 呼び出し後 → localStorage から 'token' キーが削除されている
    it('AC-13: localStorage から token キーが削除される', () => {
      const { result } = renderLoggedIn()
      act(() => {
        result.current.logout()
      })
      expect(localStorage.getItem('token')).toBeNull()
    })
  })

  describe('updateUser() 呼び出し後', () => {
    // AC-14: updateUser({ displayName: '新しい名前', ... }) 後 → user.displayName が更新される
    it('AC-14: user.displayName が更新される', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      act(() => {
        result.current.login()
      })
      act(() => {
        result.current.updateUser({
          displayName: '新しい名前',
          bio: '更新された自己紹介',
          department: '電気電子工学科',
          grade: '4年',
        })
      })
      expect(result.current.user?.displayName).toBe('新しい名前')
    })

    // AC-15: updateUser() 後も isLoggedIn は true のまま
    it('AC-15: isLoggedIn が true のまま維持される', () => {
      const { result } = renderHook(() => useAuth(), {
        wrapper: AuthProvider,
      })
      act(() => {
        result.current.login()
      })
      act(() => {
        result.current.updateUser({
          displayName: '新しい名前',
          bio: '更新された自己紹介',
          department: '電気電子工学科',
          grade: '4年',
        })
      })
      expect(result.current.isLoggedIn).toBe(true)
    })
  })
})
