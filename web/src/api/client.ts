// AT を Bearer ヘッダーに自動付与し、401 時にリフレッシュ→リトライする fetch ラッパー

type GetToken = () => string | null
type SetToken = (token: string) => void
type OnAuthFailure = () => void

let getTokenFn: GetToken = () => null
let setTokenFn: SetToken = () => {}
let onAuthFailureFn: OnAuthFailure = () => {}
let isRefreshing = false

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

  // リフレッシュ中の多重呼び出しを防ぐ
  if (isRefreshing) {
    onAuthFailureFn()
    return res
  }

  // AT リフレッシュを試みる
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
async function attemptRefresh(): Promise<string | null> {
  isRefreshing = true
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
    isRefreshing = false
  }
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
