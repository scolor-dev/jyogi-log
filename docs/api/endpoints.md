# エンドポイント一覧

## `GET /health`

サーバーおよびデータベース接続の死活確認。

| 項目 | 内容 |
|------|------|
| 認証 | 不要 |
| 主な用途 | ヘルスチェック・ロードバランサーの生存確認 |

**レスポンス**

| ステータス | 説明 |
|-----------|------|
| 200 | サーバー正常稼働中 |
| 503 | DB 接続不可等の異常あり |

---

## `POST /auth/signup`

新規ユーザーを登録し、即時ログイン状態にする。

| 項目 | 内容 |
|------|------|
| 認証 | 不要 |
| DB 操作 | `users` / `user_identities` / `user_credentials` / `user_profile` / `sessions` / `refresh_tokens` へ INSERT |

**リクエストボディ**

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:----:|------|
| `email` | string | ✓ | メールアドレス（`user_identities.identifier` に登録） |
| `password` | string | ✓ | パスワード（8文字以上。ハッシュ化して `user_credentials.secret_hash` に保存） |
| `display_name` | string | ✓ | 表示名（`user_profile.display_name` に保存） |

**バリデーション**

- `email` が `user_identities` に既存（`revoked_at IS NULL`）の場合は `409 EMAIL_ALREADY_EXISTS`
- `password` が 8 文字未満の場合は `400 VALIDATION_ERROR`

**レスポンス**

| ステータス | 説明 |
|-----------|------|
| 201 | ユーザー作成成功。アクセストークン・リフレッシュトークンを返す |
| 400 | バリデーションエラー |
| 409 | メールアドレス重複 |

---

## `POST /auth/login`

メールアドレスとパスワードで認証し、トークンを発行する。

| 項目 | 内容 |
|------|------|
| 認証 | 不要 |
| DB 操作 | `sessions` に INSERT、`refresh_tokens` に INSERT、`user_identities.last_used_at` を UPDATE |

**リクエストボディ**

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:----:|------|
| `email` | string | ✓ | メールアドレス |
| `password` | string | ✓ | パスワード |

**認証フロー**

1. `user_identities` から `type = 'email'` かつ `normalized_identifier` が一致するレコードを取得
2. `users.status` が `active` であることを確認
3. `user_credentials` から `type = 'password'` のレコードを取得し、パスワードハッシュを照合
4. `sessions` にセッションレコードを INSERT（`ip_address` / `user_agent` を記録）
5. `refresh_tokens` に token_hash を INSERT
6. アクセストークン（JWT）を生成して返す

**レスポンス**

| ステータス | 説明 |
|-----------|------|
| 200 | 認証成功。アクセストークン・リフレッシュトークンを返す |
| 400 | バリデーションエラー |
| 401 | 認証情報不正（`INVALID_CREDENTIALS`） |

---

## `POST /auth/logout`

現在のセッションを失効させる。

| 項目 | 内容 |
|------|------|
| 認証 | 必要（Bearer トークン） |
| DB 操作 | `sessions.revoked_at` を UPDATE、`refresh_tokens.revoked_at` を UPDATE |

**リクエストボディ**

なし

**ログアウトフロー**

1. Bearer トークンからセッション ID を特定
2. `sessions.revoked_at` に現在時刻を記録（セッション失効）
3. 該当セッションに紐づく `refresh_tokens` を一括失効（`revoked_at` を UPDATE）

**レスポンス**

| ステータス | 説明 |
|-----------|------|
| 204 | ログアウト成功（ボディなし） |
| 401 | トークン未付与または無効 |

---

## `POST /auth/refresh`

リフレッシュトークンを使ってアクセストークンを再発行する（ローテーション方式）。

| 項目 | 内容 |
|------|------|
| 認証 | 不要（リフレッシュトークンで代替） |
| DB 操作 | `refresh_tokens` の旧レコードを失効、新レコードを INSERT |

**リクエストボディ**

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:----:|------|
| `refresh_token` | string | ✓ | 有効なリフレッシュトークン |

**リフレッシュフロー**

1. `refresh_tokens` から `token_hash` が一致し `revoked_at IS NULL` かつ `expires_at` が未来のレコードを取得
2. 旧リフレッシュトークンの `revoked_at` に現在時刻を記録（使い捨て）
3. 新しいリフレッシュトークンを生成して `refresh_tokens` に INSERT
4. 新しいアクセストークン（JWT）を生成して返す

> **注意**: 既に失効済みのリフレッシュトークンが使われた場合、トークン再利用攻撃の可能性があるため、該当セッション全体を失効させることを推奨する。

**レスポンス**

| ステータス | 説明 |
|-----------|------|
| 200 | 更新成功。新しいアクセストークン・リフレッシュトークンを返す |
| 401 | リフレッシュトークンが無効・失効済み・期限切れ（`REFRESH_TOKEN_INVALID`） |

---

## `GET /auth/me`

現在認証中のユーザー情報を返す。

| 項目 | 内容 |
|------|------|
| 認証 | 必要（Bearer トークン） |
| DB 操作 | `users` / `user_profile` / `user_identities` を SELECT |

**リクエストボディ**

なし

**レスポンス**

| ステータス | 説明 |
|-----------|------|
| 200 | 認証中ユーザーの情報を返す |
| 401 | トークン未付与・無効・期限切れ |
