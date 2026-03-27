# ADR 0007 — refresh_tokens に sessions テーブルを導入する

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-12 |
| 関連テーブル | `sessions`, `refresh_tokens` |

> **経緯**: ADR 0002 にて `users` に直接 `refresh_tokens` を紐づける構成を決定したが、その後本 ADR の理由により `sessions` テーブルを挟む構成へ変更した。

---

## 背景

ADR 0002 の時点では `users (1) ──< refresh_tokens (多)` という構成を採用していた。  
その後、デバイス単位のセッション管理が必要と判断し `sessions` テーブルの導入を検討した。

---

## 問題

`users` に直接 `refresh_tokens` を紐づける構成では、リフレッシュトークンをデバイス（ログイン単位）で束ねる手段がない。

- 複数デバイスでログインしている場合、どのトークンがどのデバイスのものか識別できない
- 特定デバイスのみログアウトさせる（デバイス単位の失効）ができない

---

## 検討した選択肢

### A. `refresh_tokens` にデバイス識別カラムを追加する — 却下

- `refresh_tokens` に `ip_address` / `user_agent` を直接持たせてデバイスを識別する
- ローテーションで refresh_tokens レコードが毎回入れ替わるため、「どのレコードが同じログインか」を追跡する手段がない
- デバイス単位の失効を実現できない

### B. `sessions` テーブルを導入し `refresh_tokens` を紐づける — 採用

```
users (1) ──< sessions (多) ──< refresh_tokens (多)
```

- `sessions` がログイン単位の識別子となり、デバイスを特定できる
- リフレッシュトークンがローテーションで入れ替わっても `session_id` で同じログインに属することを追跡できる
- `sessions.revoked_at` を更新するだけでそのセッションに紐づく全トークンをデバイス単位で一括失効できる
- 将来の「他のデバイスからログアウト」機能もセッション単位で実現できる

---

## 決定

**選択肢 B を採用する。**

ログイン時に `sessions` レコードを INSERT し、発行する `refresh_tokens` に `session_id` を持たせる。  
`refresh_tokens.session_id` は `ON DELETE SET NULL` とし、セッション削除時もトークン履歴を保持する。

```
ログイン
  → sessions INSERT（user_uuid, ip_address, user_agent, expires_at）
  → refresh_tokens INSERT（session_id, token_hash, expires_at）

ログアウト（デバイス単位）
  → 対象 sessions.revoked_at = NOW()
  → 紐づく refresh_tokens.revoked_at = NOW()（一括失効）

全デバイスログアウト
  → user_uuid に紐づく全 sessions.revoked_at = NOW()
  → 紐づく全 refresh_tokens を一括失効
```

---

## 影響と制約

- ADR 0002 の構成（`users` 直結）から変更となる。`refresh_tokens` テーブルに `session_id` カラムを追加するマイグレーションが必要
- `sessions` の `expires_at` はリフレッシュトークンの最長有効期限と合わせる
- 期限切れセッションのクリーンアップバッチを別途実装する必要がある
- `client_tokens`（ADR 0006）はユーザーセッションに紐づかないため、この変更の影響を受けない

---

## 参考

- [OWASP — Session Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html)
- [Auth0 Blog — Refresh Token Rotation](https://auth0.com/docs/secure/tokens/refresh-tokens/refresh-token-rotation)
