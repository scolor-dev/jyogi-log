// AT を Bearer ヘッダーに自動付与し、401 時にリフレッシュ→リトライする fetch ラッパー
// TODO: 認証済みAPIは apiFetch を使うこと。loginApi/signupApi/logoutApi は AT不要のため fetch 直呼びで正しい

type GetToken = () => string | null
type SetToken = (token: string) => void
type OnAuthFailure = () => void

let getTokenFn: GetToken = () => null
let setTokenFn: SetToken = () => {}
let onAuthFailureFn: OnAuthFailure = () => {}
// 進行中のリフレッシュ Promise を保持する（並列401が同一 Promise を共有して待機するため）
let refreshPromise: Promise<string | null> | null = null

// AuthProvider の useEffect から呼ぶ初期化関数
export function initApiClient(
  getToken: GetToken,
  setToken: SetToken,
  onAuthFailure: OnAuthFailure
): void {
  getTokenFn = getToken
  setTokenFn = setToken
  onAuthFailureFn = onAuthFailure
}

// AT 付き fetch。401 時にリフレッシュ→リトライ（1回のみ）
export async function apiFetch(
  input: RequestInfo,
  init?: RequestInit
): Promise<Response> {
  const token = getTokenFn()
  const headers = buildHeaders(init?.headers, token)

  const res = await fetch(input, { ...init, headers, credentials: 'include' })

  if (res.status !== 401) {
    return res
  }

  // リフレッシュ中の並列401は同一 Promise を共有して待機する（多重リフレッシュ防止）
  const newToken = await attemptRefresh()
  if (newToken === null) {
    // リフレッシュ失敗 → 認証状態クリア
    onAuthFailureFn()
    return res
  }

  // 新 AT をメモリに上書き
  setTokenFn(newToken)

  // 新 AT でリトライ
  const retryHeaders = buildHeaders(init?.headers, newToken)
  return fetch(input, { ...init, headers: retryHeaders, credentials: 'include' })
}

// リフレッシュを実行し、成功時は新 AT を返す（失敗時は null）
// 進行中のリフレッシュがある場合は同一 Promise を返すことで並列401を1回のリフレッシュに収束させる
async function attemptRefresh(): Promise<string | null> {
  if (refreshPromise !== null) {
    return refreshPromise
  }

  refreshPromise = (async () => {
    try {
      const refreshRes = await fetch('/api/v1/auth/refresh', {
        method: 'POST',
        credentials: 'include',
      })

      if (!refreshRes.ok) {
        return null
      }

      const data = (await refreshRes.json()) as { access_token: string }
      return data.access_token
    } catch {
      return null
    } finally {
      refreshPromise = null
    }
  })()

  return refreshPromise
}

// ヘッダーを構築するユーティリティ
function buildHeaders(
  existing: RequestInit['headers'] | undefined,
  token: string | null
): Headers {
  const headers = new Headers(existing)
  if (token) {
    headers.set('Authorization', `Bearer ${token}`)
  }
  return headers
}
