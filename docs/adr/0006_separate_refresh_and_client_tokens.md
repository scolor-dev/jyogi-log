# ADR 0006 — リフレッシュトークンとクライアントトークンのテーブルを分ける

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-02 |
| 関連テーブル | `refresh_tokens`, `client_tokens` |

---

## 背景

本システムには性質の異なる2種類のトークンが存在する。

- **リフレッシュトークン**: ユーザーがログインした際に発行される。ユーザーセッションに紐づき、AccessToken の再発行に使う
- **クライアントトークン**: OAuth のクライアント・クレデンシャルズフローで発行される。ユーザーが関与しない M2M（Machine-to-Machine）通信向け

これらをどのテーブルで管理するかを検討した。

---

## 問題

両トークンは「ハッシュで保存する」「有効期限・失効管理が必要」という共通点がある一方、紐づく主体・用途・ライフサイクルが根本的に異なる。

| 観点 | リフレッシュトークン | クライアントトークン |
|------|-------------------|--------------------|
| 紐づく主体 | ユーザー（`sessions` 経由） | OAuth クライアント（`oauth_clients`） |
| 用途 | AccessToken の再発行 | M2M API アクセス |
| ローテーション | あり（使い捨て） | なし（有効期限まで使い続ける） |
| ユーザー認証 | 必要 | 不要 |

---

## 検討した選択肢

### A. 1テーブルにまとめる（`tokens` テーブル）

- `type` カラムで `refresh` / `client` を区別する
- テーブル数が減りシンプルに見える
- しかし紐づく FK が異なる（`session_id` vs `client_id`）ため、どちらか一方が必ず NULL になる
- ローテーションの有無・スコープの意味・失効ルールが異なり、`type` による条件分岐がアプリ層に増える
- インデックスやクリーンアップ処理も `type` で毎回フィルタする必要があり、見通しが悪くなる

### B. テーブルを分ける（`refresh_tokens` / `client_tokens`）— 採用

- それぞれの主体（`session_id` / `client_id`）に対して FK を明確に定義できる
- ローテーション・スコープ・失効ルールをテーブル単位で独立して設計できる
- クリーンアップバッチ・インデックス設計がそれぞれの用途に最適化できる
- テーブルが増えるが、責務が明確に分離されており見通しが良い

---

## 決定

**選択肢 B を採用する。**

`refresh_tokens` と `client_tokens` をそれぞれ独立したテーブルとして管理する。

```
sessions ──< refresh_tokens
  session_id FK（SET NULL）
  token_hash
  scope
  expires_at
  revoked_at   ← ローテーションで毎回更新

oauth_clients ──< client_tokens
  client_id FK（CASCADE）
  token_hash
  scope
  expires_at
  revoked_at   ← 有効期限切れ・明示的な失効時のみ更新
```

両テーブルとも `token_hash` に `WHERE revoked_at IS NULL` の部分インデックスを張り、有効なトークンの検索を効率化する。

---

## 影響と制約

- `refresh_tokens` はローテーション方式（ADR 0003）のため失効済みレコードも一定期間保持するが、`client_tokens` は失効後に削除して構わない
- `client_tokens` にはユーザー情報が紐づかないため、発行・失効の操作は OAuth クライアント認証のみで行う
- 期限切れレコードのクリーンアップバッチはそれぞれ独立して実装する

---

## 参考

- [RFC 6749 — Client Credentials Grant](https://datatracker.ietf.org/doc/html/rfc6749#section-4.4)
- [RFC 6819 — OAuth 2.0 Threat Model and Security Considerations](https://datatracker.ietf.org/doc/html/rfc6819)
