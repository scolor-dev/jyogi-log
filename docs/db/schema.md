# Database Schema

## 概要

PostgreSQL を使用した認証・認可基盤のスキーマ。  
ユーザー管理・セッション管理・OAuth 2.0（認可コードフロー + PKCE）・RBAC・M2M トークンに対応する。

---

## Extensions & 共通設定

| 項目 | 内容 |
|------|------|
| Extension | `pgcrypto`（UUID生成: `gen_random_uuid()`） |
| 共通トリガー | `set_updated_at()` — UPDATE 前に `updated_at` を自動更新 |

---

## テーブル一覧

| # | テーブル名 | 概要 |
|---|-----------|------|
| 01 | `users` | ユーザー基本情報 |
| 02 | `user_identities` | ログイン識別子（メール・電話等） |
| 03 | `user_credentials` | 認証手段（パスワード・TOTP・WebAuthn 等） |
| 04 | `user_profile` | プロフィール情報（1:1） |
| 05 | `sessions` | ログインセッション |
| 06 | `refresh_tokens` | リフレッシュトークン |
| 07 | `roles` | ロール定義 |
| 07 | `user_roles` | ユーザー ↔ ロール 中間テーブル |
| 08 | `oauth_clients` | OAuth クライアント |
| 08 | `oauth_client_redirect_uris` | クライアントの許可リダイレクト URI |
| 08 | `oauth_scopes` | スコープ定義 |
| 08 | `oauth_client_scopes` | クライアント ↔ スコープ 中間テーブル |
| 09 | `oauth_authorization_codes` | 認可コード（短命・使い捨て） |
| 10 | `client_tokens` | M2M サービス向けトークン |

---

## Enum 定義

### `user_status`

| 値 | 意味 |
|----|------|
| `active` | 有効（デフォルト） |
| `inactive` | 無効 |
| `suspended` | 停止中 |
| `deleted` | 削除済み |

### `oauth_client_type`

| 値 | 意味 |
|----|------|
| `public` | SPA・モバイルアプリ（`client_secret` なし） |
| `confidential` | サーバーサイドアプリ（`client_secret` あり） |

---

## テーブル詳細

### `users`
ユーザーの最小限の基本情報。識別子・認証手段・プロフィールはそれぞれ別テーブルで管理する。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK（内部結合用） |
| `uuid` | `UUID` | NO | `gen_random_uuid()` | 外部公開用 ID（UNIQUE） |
| `status` | `user_status` | NO | `active` | アカウント状態 |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |
| `deleted_at` | `TIMESTAMPTZ` | YES | — | 論理削除日時 |

**インデックス**
- `idx_users_status` — `status` で絞り込み

---

### `user_identities`
ユーザーのログイン識別子（メールアドレス・電話番号・OAuth サブジェクト等）を管理する。  
複数の識別子を持てるが、有効な primary は 1 件のみ。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK |
| `user_id` | `BIGINT` | NO | — | FK → `users.id`（CASCADE） |
| `type` | `VARCHAR(50)` | NO | — | 識別子の種別（例: `email`, `phone`, `oauth_google`） |
| `identifier` | `VARCHAR(255)` | NO | — | 識別子の元の値 |
| `normalized_identifier` | `VARCHAR(255)` | NO | — | 正規化後の値（重複チェック用） |
| `is_primary` | `BOOLEAN` | NO | `FALSE` | プライマリ識別子フラグ |
| `last_used_at` | `TIMESTAMPTZ` | YES | — | 最終使用日時 |
| `verified_at` | `TIMESTAMPTZ` | YES | — | 検証済み日時 |
| `revoked_at` | `TIMESTAMPTZ` | YES | — | 無効化日時（soft delete） |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**制約**
- `identifier` / `normalized_identifier` は空文字不可（CHECK）

**インデックス**
- `ux_user_identities_active_identifier` — `(type, normalized_identifier)` UNIQUE（`revoked_at IS NULL` のみ対象）
- `idx_user_identities_user_id` — `user_id`
- `ux_user_identities_primary` — `user_id` UNIQUE（`is_primary = TRUE AND revoked_at IS NULL` のみ対象）

---

### `user_credentials`
ユーザーの認証手段（パスワード・TOTP・WebAuthn・OAuth リフレッシュ等）を管理する。  
`secret_hash` には生の値を入れず、必ずハッシュを格納する。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK |
| `user_id` | `BIGINT` | NO | — | FK → `users.id`（CASCADE） |
| `type` | `VARCHAR(50)` | NO | — | 認証手段の種別（例: `password`, `totp`, `webauthn`, `oauth_refresh`） |
| `secret_hash` | `TEXT` | NO | — | ハッシュ化済みシークレット |
| `secret_meta` | `JSONB` | YES | — | 公開鍵・設定情報等の補足データ |
| `is_primary` | `BOOLEAN` | NO | `FALSE` | プライマリ認証手段フラグ |
| `last_used_at` | `TIMESTAMPTZ` | YES | — | 最終使用日時 |
| `verified_at` | `TIMESTAMPTZ` | YES | — | 検証済み日時 |
| `revoked_at` | `TIMESTAMPTZ` | YES | — | 無効化日時 |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**インデックス**
- `idx_user_credentials_user_id` — `user_id`
- `idx_user_credentials_type` — `type`
- `ux_user_credentials_primary` — `user_id` UNIQUE（`is_primary = TRUE AND revoked_at IS NULL` のみ対象）

---

### `user_profile`
`users` と 1:1 対応するプロフィール情報。PK が `user_id` を兼ねる。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `user_id` | `BIGINT` | NO | — | PK & FK → `users.id`（CASCADE） |
| `display_name` | `VARCHAR(255)` | NO | — | 表示名 |
| `avatar_url` | `VARCHAR(512)` | YES | — | アバター画像 URL |
| `tagline` | `VARCHAR(100)` | YES | — | 一言紹介（最大 100 文字） |
| `bio` | `TEXT` | YES | — | プロフィール本文 |
| `locale` | `VARCHAR(50)` | YES | — | ロケール（例: `ja`, `en-US`） |
| `timezone` | `VARCHAR(50)` | YES | — | タイムゾーン（例: `Asia/Tokyo`） |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**制約**
- `tagline` は 100 文字以下（CHECK）

---

### `sessions`
ユーザーのログインセッション。外部参照は `users.uuid`（公開 ID）を使用する。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK |
| `user_uuid` | `UUID` | NO | — | FK → `users.uuid`（CASCADE） |
| `ip_address` | `INET` | YES | — | ログイン時の IP アドレス |
| `user_agent` | `TEXT` | YES | — | ログイン時の User-Agent |
| `last_used_at` | `TIMESTAMPTZ` | YES | — | 最終アクティブ日時 |
| `expires_at` | `TIMESTAMPTZ` | NO | — | セッション有効期限 |
| `revoked_at` | `TIMESTAMPTZ` | YES | — | 強制失効日時 |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**インデックス**
- `idx_sessions_user_uuid` — `user_uuid`
- `idx_sessions_expires_at` — 期限切れセッションのクリーンアップ用

---

### `refresh_tokens`
セッションに紐づくリフレッシュトークン。トークン本体は必ずハッシュで格納する。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK |
| `session_id` | `BIGINT` | YES | — | FK → `sessions.id`（SET NULL） |
| `token_hash` | `TEXT` | NO | — | ハッシュ化済みリフレッシュトークン |
| `scope` | `TEXT` | YES | — | 付与スコープ（スペース区切り） |
| `last_used_at` | `TIMESTAMPTZ` | YES | — | 最終使用日時 |
| `expires_at` | `TIMESTAMPTZ` | NO | — | トークン有効期限 |
| `revoked_at` | `TIMESTAMPTZ` | YES | — | 失効日時 |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**インデックス**
- `ux_refresh_tokens_token_hash` — `token_hash` UNIQUE（`revoked_at IS NULL` のみ対象）
- `idx_refresh_tokens_session_id` — `session_id`
- `idx_refresh_tokens_expires_at` — 期限切れトークンのクリーンアップ用

---

### `roles`
権限の定義テーブル。権限はブールカラムで直接管理する（中間テーブルなし）。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK |
| `name` | `VARCHAR(50)` | NO | — | ロール名（UNIQUE） |
| `description` | `TEXT` | YES | — | ロールの説明 |
| `is_super_admin` | `BOOLEAN` | NO | `FALSE` | スーパー管理者権限 |
| `can_manage_users` | `BOOLEAN` | NO | `FALSE` | ユーザー管理権限 |
| `can_manage_roles` | `BOOLEAN` | NO | `FALSE` | ロール管理権限 |
| `can_manage_clients` | `BOOLEAN` | NO | `FALSE` | OAuthクライアント管理権限 |
| `can_manage_scopes` | `BOOLEAN` | NO | `FALSE` | スコープ管理権限 |
| `can_view_audit_logs` | `BOOLEAN` | NO | `FALSE` | 監査ログ閲覧権限 |
| `can_view_auth_events` | `BOOLEAN` | NO | `FALSE` | 認証イベント閲覧権限 |
| `can_revoke_tokens` | `BOOLEAN` | NO | `FALSE` | トークン失効権限 |
| `can_revoke_sessions` | `BOOLEAN` | NO | `FALSE` | セッション失効権限 |
| `can_view_users` | `BOOLEAN` | NO | `FALSE` | ユーザー閲覧権限 |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**インデックス**
- `idx_roles_name` — `name`

---

### `user_roles`
ユーザーとロールの多対多中間テーブル。

| カラム | 型 | NULL | 説明 |
|--------|----|------|------|
| `user_id` | `BIGINT` | NO | PK & FK → `users.id`（CASCADE） |
| `role_id` | `BIGINT` | NO | PK & FK → `roles.id`（CASCADE） |
| `created_at` | `TIMESTAMPTZ` | NO | 付与日時 |

**インデックス**
- `idx_user_roles_user_id` — `user_id`
- `idx_user_roles_role_id` — `role_id`

---

### `oauth_clients`
OAuth 2.0 クライアントの管理テーブル。`public` / `confidential` の 2 種類をサポートする。

| カラム | 型 | NULL | デフォルト | 説明 |
|--------|----|------|-----------|------|
| `id` | `BIGSERIAL` | NO | — | PK |
| `client_id` | `VARCHAR(255)` | NO | — | クライアント識別子（UNIQUE） |
| `client_name` | `VARCHAR(255)` | NO | — | クライアント表示名 |
| `client_type` | `oauth_client_type` | NO | — | `public` or `confidential` |
| `client_secret_hash` | `TEXT` | YES | — | ハッシュ化済みシークレット（`confidential` のみ） |
| `revoked_at` | `TIMESTAMPTZ` | YES | — | 失効日時 |
| `created_at` | `TIMESTAMPTZ` | NO | `NOW()` | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | `NOW()` | 更新日時（自動更新） |

**制約**
- `public` → `client_secret_hash IS NULL`、`confidential` → `client_secret_hash IS NOT NULL`（CHECK）

**インデックス**
- `idx_oauth_clients_client_id` — `client_id`（`revoked_at IS NULL` のみ対象）

---

### `oauth_client_redirect_uris`
クライアントに許可するリダイレクト URI のホワイトリスト。

| カラム | 型 | NULL | 説明 |
|--------|----|------|------|
| `id` | `BIGSERIAL` | NO | PK |
| `client_id` | `BIGINT` | NO | FK → `oauth_clients.id`（CASCADE） |
| `redirect_uri` | `TEXT` | NO | 許可リダイレクト URI |
| `created_at` | `TIMESTAMPTZ` | NO | 作成日時 |

**インデックス**
- `ux_oauth_client_redirect_uris` — `(client_id, redirect_uri)` UNIQUE
- `idx_oauth_client_redirect_uris_client_id` — `client_id`

---

### `oauth_scopes`
利用可能なスコープの定義テーブル。

| カラム | 型 | NULL | 説明 |
|--------|----|------|------|
| `id` | `BIGSERIAL` | NO | PK |
| `name` | `VARCHAR(255)` | NO | スコープ名（UNIQUE） |
| `description` | `TEXT` | YES | スコープの説明 |
| `created_at` | `TIMESTAMPTZ` | NO | 作成日時 |

---

### `oauth_client_scopes`
クライアントに許可するスコープの多対多中間テーブル。

| カラム | 型 | NULL | 説明 |
|--------|----|------|------|
| `client_id` | `BIGINT` | NO | PK & FK → `oauth_clients.id`（CASCADE） |
| `scope_id` | `BIGINT` | NO | PK & FK → `oauth_scopes.id`（CASCADE） |
| `created_at` | `TIMESTAMPTZ` | NO | 作成日時 |

**インデックス**
- `idx_oauth_client_scopes_client_id` — `client_id`

---

### `oauth_authorization_codes`
認可コードフローで発行される短命・使い捨てのコード。PKCE（S256 / plain）対応。

| カラム | 型 | NULL | 説明 |
|--------|----|------|------|
| `id` | `BIGSERIAL` | NO | PK |
| `user_id` | `BIGINT` | NO | FK → `users.id`（CASCADE） |
| `client_id` | `BIGINT` | NO | FK → `oauth_clients.id`（CASCADE） |
| `session_id` | `BIGINT` | YES | FK → `sessions.id`（SET NULL） |
| `code_hash` | `TEXT` | NO | ハッシュ化済み認可コード |
| `code_challenge` | `TEXT` | YES | PKCE コードチャレンジ |
| `code_challenge_method` | `VARCHAR(10)` | YES | PKCE メソッド（`S256` or `plain`） |
| `redirect_uri` | `TEXT` | NO | リダイレクト先 URI |
| `scope` | `TEXT` | YES | 要求スコープ（スペース区切り） |
| `expires_at` | `TIMESTAMPTZ` | NO | コード有効期限 |
| `consumed_at` | `TIMESTAMPTZ` | YES | 使用済み日時（一度使ったら記録） |
| `created_at` | `TIMESTAMPTZ` | NO | 作成日時 |

**インデックス**
- `ux_oauth_authorization_codes_code_hash` — `code_hash` UNIQUE（`consumed_at IS NULL` のみ対象）
- `idx_oauth_authorization_codes_user_id` — `user_id`
- `idx_oauth_authorization_codes_client_id` — `client_id`
- `idx_oauth_authorization_codes_expires_at` — 期限切れコードのクリーンアップ用

---

### `client_tokens`
M2M（Machine-to-Machine）サービス向けのクライアントクレデンシャルトークン。

| カラム | 型 | NULL | 説明 |
|--------|----|------|------|
| `id` | `BIGSERIAL` | NO | PK |
| `client_id` | `BIGINT` | NO | FK → `oauth_clients.id`（CASCADE） |
| `token_hash` | `TEXT` | NO | ハッシュ化済みトークン |
| `scope` | `TEXT` | YES | 付与スコープ（スペース区切り） |
| `last_used_at` | `TIMESTAMPTZ` | YES | 最終使用日時 |
| `expires_at` | `TIMESTAMPTZ` | NO | トークン有効期限 |
| `revoked_at` | `TIMESTAMPTZ` | YES | 失効日時 |
| `created_at` | `TIMESTAMPTZ` | NO | 作成日時 |
| `updated_at` | `TIMESTAMPTZ` | NO | 更新日時（自動更新） |

**インデックス**
- `ux_client_tokens_token_hash` — `token_hash` UNIQUE（`revoked_at IS NULL` のみ対象）
- `idx_client_tokens_client_id` — `client_id`
- `idx_client_tokens_expires_at` — 期限切れトークンのクリーンアップ用

---

## ER 図

```mermaid
erDiagram

  users {
    bigserial id PK
    uuid uuid UK
    user_status status
    timestamptz created_at
    timestamptz updated_at
    timestamptz deleted_at
  }

  user_identities {
    bigserial id PK
    bigint user_id FK
    varchar type
    varchar identifier
    varchar normalized_identifier
    boolean is_primary
    timestamptz last_used_at
    timestamptz verified_at
    timestamptz revoked_at
    timestamptz created_at
    timestamptz updated_at
  }

  user_credentials {
    bigserial id PK
    bigint user_id FK
    varchar type
    text secret_hash
    jsonb secret_meta
    boolean is_primary
    timestamptz last_used_at
    timestamptz verified_at
    timestamptz revoked_at
    timestamptz created_at
    timestamptz updated_at
  }

  user_profile {
    bigint user_id PK_FK
    varchar display_name
    varchar avatar_url
    varchar tagline
    text bio
    varchar locale
    varchar timezone
    timestamptz created_at
    timestamptz updated_at
  }

  sessions {
    bigserial id PK
    uuid user_uuid FK
    inet ip_address
    text user_agent
    timestamptz last_used_at
    timestamptz expires_at
    timestamptz revoked_at
    timestamptz created_at
    timestamptz updated_at
  }

  refresh_tokens {
    bigserial id PK
    bigint session_id FK
    text token_hash
    text scope
    timestamptz last_used_at
    timestamptz expires_at
    timestamptz revoked_at
    timestamptz created_at
    timestamptz updated_at
  }

  roles {
    bigserial id PK
    varchar name UK
    text description
    boolean is_super_admin
    boolean can_manage_users
    boolean can_manage_roles
    boolean can_manage_clients
    boolean can_manage_scopes
    boolean can_view_audit_logs
    boolean can_view_auth_events
    boolean can_revoke_tokens
    boolean can_revoke_sessions
    boolean can_view_users
    timestamptz created_at
    timestamptz updated_at
  }

  user_roles {
    bigint user_id PK_FK
    bigint role_id PK_FK
    timestamptz created_at
  }

  oauth_clients {
    bigserial id PK
    varchar client_id UK
    varchar client_name
    oauth_client_type client_type
    text client_secret_hash
    timestamptz revoked_at
    timestamptz created_at
    timestamptz updated_at
  }

  oauth_client_redirect_uris {
    bigserial id PK
    bigint client_id FK
    text redirect_uri
    timestamptz created_at
  }

  oauth_scopes {
    bigserial id PK
    varchar name UK
    text description
    timestamptz created_at
  }

  oauth_client_scopes {
    bigint client_id PK_FK
    bigint scope_id PK_FK
    timestamptz created_at
  }

  oauth_authorization_codes {
    bigserial id PK
    bigint user_id FK
    bigint client_id FK
    bigint session_id FK
    text code_hash
    text code_challenge
    varchar code_challenge_method
    text redirect_uri
    text scope
    timestamptz expires_at
    timestamptz consumed_at
    timestamptz created_at
  }

  client_tokens {
    bigserial id PK
    bigint client_id FK
    text token_hash
    text scope
    timestamptz last_used_at
    timestamptz expires_at
    timestamptz revoked_at
    timestamptz created_at
    timestamptz updated_at
  }

  users ||--o{ user_identities : "has"
  users ||--o{ user_credentials : "has"
  users ||--o| user_profile : "has"
  users ||--o{ sessions : "has"
  users ||--o{ user_roles : "assigned"
  users ||--o{ oauth_authorization_codes : "authorizes"

  roles ||--o{ user_roles : "assigned"

  sessions ||--o{ refresh_tokens : "has"
  sessions ||--o{ oauth_authorization_codes : "bound to"

  oauth_clients ||--o{ oauth_client_redirect_uris : "has"
  oauth_clients ||--o{ oauth_client_scopes : "allows"
  oauth_clients ||--o{ oauth_authorization_codes : "issued to"
  oauth_clients ||--o{ client_tokens : "has"

  oauth_scopes ||--o{ oauth_client_scopes : "allowed by"
```

---

## 設計上のポイント

**ハッシュ必須カラム**  
`secret_hash`（user_credentials）、`token_hash`（refresh_tokens, client_tokens）、`code_hash`（oauth_authorization_codes）、`client_secret_hash`（oauth_clients）はすべてハッシュ値のみ格納する。生の値はいかなる場合も保存しない。

**Soft delete パターン**  
`revoked_at`（identities, credentials, sessions, tokens, clients）および `deleted_at`（users）で論理削除を行う。有効レコードだけを対象とする部分インデックス（`WHERE revoked_at IS NULL`）により検索効率を維持する。

**内部 ID と公開 ID の分離**  
`users.id`（BIGSERIAL）は内部結合専用、`users.uuid`（UUID）を外部公開用として使い分ける。`sessions` は `user_uuid` で参照しており、内部 ID を外部に露出しない。

**使い捨て認可コード**  
`oauth_authorization_codes.consumed_at` に使用済み日時を記録する。部分インデックスにより未使用コードのみ UNIQUE 制約が適用され、再利用を防ぐ。

**M2M トークン**  
`client_tokens` はユーザーに紐づかない。OAuth クライアントクレデンシャルフローで発行されるサービス間通信用トークン。

**自動更新トリガー**  
`set_updated_at()` 関数によるトリガーが全主要テーブルに設定されており、`updated_at` の手動管理が不要。