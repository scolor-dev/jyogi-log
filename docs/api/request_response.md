# リクエスト / レスポンス例

---

## `GET /health`

### Request

```http
GET /api/v1/health
```

### Response `200 OK`

```json
{
  "data": {
    "status": "ok",
    "timestamp": "2025-04-01T09:00:00Z"
  }
}
```

### Response `503 Service Unavailable`

```json
{
  "error": {
    "code": "INTERNAL_SERVER_ERROR",
    "message": "データベースへの接続に失敗しました。"
  }
}
```

---

## `POST /auth/signup`

### Request

```http
POST /api/v1/auth/signup
Content-Type: application/json
```

```json
{
  "email": "user@example.com",
  "password": "password1234",
  "display_name": "山田 太郎"
}
```

### Response `201 Created`

```json
{
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "dGhpcyBpcyBhIHJlZnJlc2ggdG9rZW4...",
    "token_type": "Bearer",
    "expires_in": 900,
    "user": {
      "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "email": "user@example.com",
      "display_name": "山田 太郎",
      "status": "active",
      "created_at": "2025-04-01T09:00:00Z"
    }
  }
}
```

### Response `400 Bad Request` — バリデーションエラー

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "入力内容に誤りがあります。",
    "details": [
      {
        "field": "password",
        "message": "パスワードは8文字以上で入力してください。"
      }
    ]
  }
}
```

### Response `409 Conflict` — メールアドレス重複

```json
{
  "error": {
    "code": "EMAIL_ALREADY_EXISTS",
    "message": "このメールアドレスはすでに登録されています。"
  }
}
```

---

## `POST /auth/login`

### Request

```http
POST /api/v1/auth/login
Content-Type: application/json
```

```json
{
  "email": "user@example.com",
  "password": "password1234"
}
```

### Response `200 OK`

```json
{
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "dGhpcyBpcyBhIHJlZnJlc2ggdG9rZW4...",
    "token_type": "Bearer",
    "expires_in": 900,
    "user": {
      "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "email": "user@example.com",
      "display_name": "山田 太郎",
      "status": "active",
      "created_at": "2025-04-01T09:00:00Z"
    }
  }
}
```

### Response `401 Unauthorized` — 認証情報不正

```json
{
  "error": {
    "code": "INVALID_CREDENTIALS",
    "message": "メールアドレスまたはパスワードが正しくありません。"
  }
}
```

---

## `POST /auth/logout`

### Request

```http
POST /api/v1/auth/logout
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Response `204 No Content`

ボディなし。

### Response `401 Unauthorized` — トークン無効

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "認証が必要です。"
  }
}
```

---

## `POST /auth/refresh`

### Request

```http
POST /api/v1/auth/refresh
Content-Type: application/json
```

```json
{
  "refresh_token": "dGhpcyBpcyBhIHJlZnJlc2ggdG9rZW4..."
}
```

### Response `200 OK`

```json
{
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "bmV3UmVmcmVzaFRva2Vu...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

> リフレッシュトークンはローテーション方式のため、レスポンスで返された新しいトークンに必ず差し替えること。

### Response `401 Unauthorized` — リフレッシュトークン無効

```json
{
  "error": {
    "code": "REFRESH_TOKEN_INVALID",
    "message": "リフレッシュトークンが無効または期限切れです。再度ログインしてください。"
  }
}
```

---

## `GET /auth/me`

### Request

```http
GET /api/v1/auth/me
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Response `200 OK`

```json
{
  "data": {
    "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "email": "user@example.com",
    "status": "active",
    "profile": {
      "display_name": "山田 太郎",
      "avatar_url": "https://example.com/avatars/yamada.png",
      "tagline": "エンジニア見習い",
      "bio": "Goが好きです。",
      "locale": "ja",
      "timezone": "Asia/Tokyo"
    },
    "created_at": "2025-04-01T09:00:00Z",
    "updated_at": "2025-04-01T09:00:00Z"
  }
}
```

> `profile` フィールドは `user_profile` レコードが存在しない場合 `null` を返す。

### Response `401 Unauthorized` — トークン期限切れ

```json
{
  "error": {
    "code": "TOKEN_EXPIRED",
    "message": "アクセストークンの有効期限が切れています。"
  }
}
```

### Response `401 Unauthorized` — トークン未付与

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "認証が必要です。"
  }
}
```
