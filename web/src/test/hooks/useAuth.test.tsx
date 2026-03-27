import { render, renderHook, act, waitFor } from '@testing-library/react'
import type { ReactNode } from 'react'
import { MemoryRouter } from 'react-router-dom'
import { AuthProvider, type User } from '../../stores/authContext'
import { useAuth } from '../../hooks/useAuth'
import * as authApi from '../../api/authApi'

// AuthProvider を MemoryRouter でラップするラッパー
function Wrapper({ children }: { children: ReactNode }) {
  return <MemoryRouter>{children}</MemoryRouter>
}

function AuthWrapper({ children }: { children: ReactNode }) {
  return (
    <MemoryRouter>
      <AuthProvider>{children}</AuthProvider>
    </MemoryRouter>
  )
}

const MOCK_USER: User = {
  uuid: 'test-uuid-1234',
  identifier: 'test@example.com',
  display_name: 'テストユーザー',
}

const MOCK_TOKEN = 'mock-access-token'

beforeEach(() => {
  // デフォルト: /auth/refresh が 401 を返す（未ログイン状態）
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
    ok: false,
    status: 401,
    json: () => Promise.resolve({ error: 'Unauthorized' }),
  }))
  // logoutApi をスタブ（logout テストで fetch が呼ばれても問題ないように）
  vi.spyOn(authApi, 'logoutApi').mockResolvedValue(undefined)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
})

describe('useAuth / AuthContext', () => {
  // AC-17: AuthProvider 外で useAuth() を呼ぶ → エラーがスローされる
  it('AC-17: AuthProvider 外で useAuth() を呼ぶとエラーをスローする', () => {
    expect(() => {
      renderHook(() => useAuth(), { wrapper: Wrapper })
    }).toThrow('useAuth は AuthProvider の内側で使用してください')
  })

  // AC-1: User 型が uuid / identifier / display_name フィールドを持つ
  it('AC-1: User型が uuid / identifier / display_name フィールドを持つ', async () => {
    const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

    // 初期化完了まで待機
    await waitFor(() => {
      expect(result.current.isInitializing).toBe(false)
    })

    // login で User をセットしてフィールドを確認
    act(() => {
      result.current.login(MOCK_USER, MOCK_TOKEN)
    })

    expect(result.current.user).toEqual({
      uuid: 'test-uuid-1234',
      identifier: 'test@example.com',
      display_name: 'テストユーザー',
    })
    // 旧フィールドが存在しないことを確認
    expect(result.current.user).not.toHaveProperty('displayName')
    expect(result.current.user).not.toHaveProperty('bio')
    expect(result.current.user).not.toHaveProperty('department')
    expect(result.current.user).not.toHaveProperty('grade')
  })

  describe('login(user, token) 呼び出し後', () => {
    // AC-2: login(user, token) 呼び出し後に isLoggedIn が true になる
    it('AC-2: login(user, token) 呼び出し後に isLoggedIn が true になる', async () => {
      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(result.current.isInitializing).toBe(false)
      })

      act(() => {
        result.current.login(MOCK_USER, MOCK_TOKEN)
      })

      expect(result.current.isLoggedIn).toBe(true)
    })

    // AC-2: login(user, token) 呼び出し後に user が設定される
    it('AC-2: login(user, token) 呼び出し後に user が設定される', async () => {
      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(result.current.isInitializing).toBe(false)
      })

      act(() => {
        result.current.login(MOCK_USER, MOCK_TOKEN)
      })

      expect(result.current.user).toEqual(MOCK_USER)
    })

    // AC-2: getAccessToken() が login で渡した token を返す
    it('AC-2: getAccessToken() が login で渡した token を返す', async () => {
      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(result.current.isInitializing).toBe(false)
      })

      act(() => {
        result.current.login(MOCK_USER, MOCK_TOKEN)
      })

      expect(result.current.getAccessToken()).toBe(MOCK_TOKEN)
    })
  })

  describe('logout() 呼び出し後', () => {
    async function renderLoggedIn() {
      const rendered = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(rendered.result.current.isInitializing).toBe(false)
      })

      act(() => {
        rendered.result.current.login(MOCK_USER, MOCK_TOKEN)
      })

      expect(rendered.result.current.isLoggedIn).toBe(true)
      return rendered
    }

    // AC-3: logout() 呼び出し後に isLoggedIn が false になる
    it('AC-3: logout() 呼び出し後に isLoggedIn が false になる', async () => {
      const { result } = await renderLoggedIn()

      await act(async () => {
        await result.current.logout()
      })

      expect(result.current.isLoggedIn).toBe(false)
    })

    // AC-3: logout() 呼び出し後に user が null になる
    it('AC-3: logout() 呼び出し後に user が null になる', async () => {
      const { result } = await renderLoggedIn()

      await act(async () => {
        await result.current.logout()
      })

      expect(result.current.user).toBeNull()
    })

    // AC-3: logout() 呼び出し後に getAccessToken() が null を返す
    it('AC-3: logout() 呼び出し後に getAccessToken() が null を返す', async () => {
      const { result } = await renderLoggedIn()

      await act(async () => {
        await result.current.logout()
      })

      expect(result.current.getAccessToken()).toBeNull()
    })
  })

  // AC-4: login 後も localStorage にトークンが保存されない
  it('AC-4: login 後も localStorage にトークンが保存されない', async () => {
    const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

    await waitFor(() => {
      expect(result.current.isInitializing).toBe(false)
    })

    act(() => {
      result.current.login(MOCK_USER, MOCK_TOKEN)
    })

    // localStorage にトークンが保存されていないことを確認
    expect(localStorage.getItem('token')).toBeNull()
    expect(localStorage.length).toBe(0)
  })

  describe('セッション復元（restoreSession）', () => {
    // AC-13: マウント時に /auth/refresh が成功した場合に user がセットされる
    it('AC-13: マウント時に /auth/refresh が成功した場合に user がセットされる', async () => {
      // /auth/refresh → 200 + access_token
      // /auth/me → 200 + user（getMeApi を直接スタブ）
      vi.unstubAllGlobals()
      vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
        ok: true,
        status: 200,
        json: () => Promise.resolve({ access_token: MOCK_TOKEN }),
      }))
      vi.spyOn(authApi, 'getMeApi').mockResolvedValue(MOCK_USER)

      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(result.current.isInitializing).toBe(false)
      })

      expect(result.current.user).toEqual(MOCK_USER)
      expect(result.current.isLoggedIn).toBe(true)
    })

    // AC-14: マウント直後は isInitializing が true である
    it('AC-14: マウント直後は isInitializing が true である', () => {
      // fetch が解決されないように pending のままにする
      vi.unstubAllGlobals()
      vi.stubGlobal('fetch', vi.fn().mockReturnValue(new Promise(() => { /* pending */ })))

      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      // 最初のレンダリングで isInitializing は true
      expect(result.current.isInitializing).toBe(true)
    })

    // AC-14: /auth/refresh の応答後に isInitializing が false になる
    it('AC-14: /auth/refresh の応答後に isInitializing が false になる', async () => {
      // デフォルトの fetch mock（beforeEach で 401 をセット済み）
      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(result.current.isInitializing).toBe(false)
      })

      expect(result.current.isInitializing).toBe(false)
    })

    // AC-15: /auth/refresh が 401 の場合に user が null のまま
    it('AC-15: /auth/refresh が 401 の場合に user が null のまま', async () => {
      // beforeEach で 401 が設定済み
      const { result } = renderHook(() => useAuth(), { wrapper: AuthWrapper })

      await waitFor(() => {
        expect(result.current.isInitializing).toBe(false)
      })

      expect(result.current.user).toBeNull()
      expect(result.current.isLoggedIn).toBe(false)
    })
  })
})
