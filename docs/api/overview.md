# API 概要

## ベース URL

```
/api/v1
```

---

## 認証方式

### アクセストークン（JWT）

保護されたエンドポイントへのリクエストには、`Authorization` ヘッダーに Bearer トークンを付与する。

```
Authorization: Bearer <access_token>
```

- トークンは `POST /auth/login` または `POST /auth/refresh` で取得する。
- 有効期限が短く設定されており、期限切れ後は refresh token で再取得が必要。

### リフレッシュトークン

- `POST /auth/login` のレスポンスで `refresh_token` を返す。
- `POST /auth/refresh` に渡すことで新しいアクセストークンとリフレッシュトークンを取得できる（ローテーション方式）。
- サーバー側では `refresh_tokens` テーブルの `token_hash` と照合する。有効な `revoked_at IS NULL` のレコードのみ使用可能。
- 使用済みトークンは即時 `revoked_at` を記録し、再利用不可にする。

---

## セッション管理

ログイン成功時に `sessions` テーブルへレコードが作成される。  
セッションには `ip_address` / `user_agent` が記録され、有効期限（`expires_at`）と強制失効（`revoked_at`）で管理される。

`POST /auth/logout` を呼ぶと該当セッションと紐づく refresh token が失効する。

---

## エンドポイント概要

| メソッド | パス | 認証 | 概要 |
|---------|------|:----:|------|
| `GET` | `/health` | 不要 | サーバー死活確認 |
| `POST` | `/auth/signup` | 不要 | 新規ユーザー登録 |
| `POST` | `/auth/login` | 不要 | ログイン・トークン発行 |
| `POST` | `/auth/logout` | 必要 | ログアウト・セッション失効 |
| `POST` | `/auth/refresh` | 不要 | アクセストークン更新 |
| `GET` | `/auth/me` | 必要 | 認証中ユーザー情報取得 |

---

## 共通レスポンス形式

### 成功

```json
{
  "data": { ... }
}
```

### エラー

```json
{
  "error": {
    "code": "INVALID_CREDENTIALS",
    "message": "メールアドレスまたはパスワードが正しくありません。"
  }
}
```

---

## 共通エラーコード

| HTTP | code | 説明 |
|------|------|------|
| 400 | `VALIDATION_ERROR` | リクエストパラメータが不正 |
| 401 | `UNAUTHORIZED` | トークン未付与・無効 |
| 401 | `INVALID_CREDENTIALS` | メールアドレスまたはパスワード不正 |
| 401 | `TOKEN_EXPIRED` | アクセストークン期限切れ |
| 401 | `REFRESH_TOKEN_INVALID` | リフレッシュトークン無効・失効済み |
| 403 | `FORBIDDEN` | 権限不足 |
| 404 | `NOT_FOUND` | リソースが存在しない |
| 409 | `EMAIL_ALREADY_EXISTS` | 登録済みのメールアドレス |
| 500 | `INTERNAL_SERVER_ERROR` | サーバー内部エラー |

---

## 関連テーブル

| テーブル | 役割 |
|---------|------|
| `users` | ユーザー基本情報・アカウント状態 |
| `user_identities` | メールアドレス等のログイン識別子 |
| `user_credentials` | パスワードハッシュ等の認証手段 |
| `user_profile` | 表示名・アバター等のプロフィール |
| `sessions` | ログインセッション |
| `refresh_tokens` | リフレッシュトークン（ハッシュ管理） |
