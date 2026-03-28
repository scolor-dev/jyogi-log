import type { User } from '../types/user'

export type LoginResponse = {
  user: User
  access_token: string
}

// エラーレスポンスの型
type ApiErrorBody = {
  message?: string
  detail?: string
}

// POST /api/v1/auth/login
// バックエンドは { access_token } のみ返すため、続けて getMeApi でユーザー情報を取得する
export async function loginApi(
  email: string,
  password: string
): Promise<LoginResponse> {
  const res = await fetch('/api/v1/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    // バックエンドは identifier フィールドを要求（UI 上は email として表示）
    body: JSON.stringify({ identifier: email, password }),
  })

  if (!res.ok) {
    const body: ApiErrorBody = await res.json().catch(() => ({}))
    throw new ApiError(res.status, body.message ?? body.detail ?? 'ログインに失敗しました')
  }

  const data = (await res.json()) as { access_token: string }
  const access_token = data.access_token

  // AT を使ってユーザー情報を取得する（バックエンドの login は user を返さないため）
  const user = await getMeApi(access_token)

  return { user, access_token }
}

// POST /api/v1/auth/signup
// バックエンドは { user_uuid } のみ返すため、続けて loginApi を呼んで AT とユーザー情報を取得する
export async function signupApi(
  email: string,
  displayName: string,
  password: string
): Promise<LoginResponse> {
  const res = await fetch('/api/v1/auth/signup', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    // バックエンドは identifier フィールドを要求（UI 上は email として表示）
    body: JSON.stringify({ identifier: email, display_name: displayName, password }),
  })

  if (!res.ok) {
    const body: ApiErrorBody = await res.json().catch(() => ({}))
    throw new ApiError(res.status, body.message ?? body.detail ?? '新規登録に失敗しました')
  }

  // サインアップ成功後にログインして AT とユーザー情報を取得する
  // login フェーズが失敗した場合は status=0 で throw する（signup 自体は成功済みのため）
  try {
    return await loginApi(email, password)
  } catch {
    throw new ApiError(0, '登録は完了しましたが自動ログインに失敗しました。ログイン画面からサインインしてください。')
  }
}

// POST /api/v1/auth/logout
// バックエンドは CookieJar から RT を取得するため Authorization ヘッダー不要
export async function logoutApi(): Promise<void> {
  await fetch('/api/v1/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })
  // 成否によらずクライアント状態クリアするため戻り値は使わない
}

// GET /api/v1/auth/me（Bearer ヘッダー付き）
// バックエンドの MeResponse { user_uuid, display_name, identifier } を User 型にマッピングする
export async function getMeApi(accessToken: string): Promise<User> {
  const res = await fetch('/api/v1/auth/me', {
    method: 'GET',
    headers: { Authorization: `Bearer ${accessToken}` },
    credentials: 'include',
  })

  if (!res.ok) {
    throw new ApiError(res.status, 'ユーザー情報の取得に失敗しました')
  }

  const raw = (await res.json()) as { user_uuid: string; display_name: string; identifier: string }
  return { uuid: raw.user_uuid, identifier: raw.identifier, display_name: raw.display_name }
}

// API エラー判定用クラス
export class ApiError extends Error {
  constructor(
    public readonly status: number,
    message: string
  ) {
    super(message)
    this.name = 'ApiError'
  }
}
