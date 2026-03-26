# 機能一覧

---

## 概要
- システム名： Jyogi OAuth
- 目的： 部員の認証、一元管理 / OAuth Providerとしての認可基盤提供
- 想定ユーザー： 情報技術部部員 / 部内サービス

---

## 機能一覧

**カテゴリ一覧**
- ユーザー
- 認証
- OAuth Provider
- セッション
- 権限

---

### ユーザー
| ID | 機能名 | 概要 | 優先度 | 備考 |
|----|--------|------|--------|------|
| User-0001 | ユーザー作成 | username + password によるユーザー登録 | 🔴 P0 |  |
| User-0002 | ユーザー状態管理 | `pending`,`active`,`suspended`,`banned` の状態管理 | 🔴 P0 |  |
| User-0003 | プロフィール取得 | ユーザー情報の取得 | 🔴 P0 |  |
| User-0004 | プロフィール更新 | ユーザー情報の編集 | 🔴 P0 |  |
| User-0005 | 論理削除 | deleted_at によるソフトデリート | 🔴 P0 |  |
| User-0006 | ユーザー検索 | username / id による検索 | 🟢 P2 |  |
| User-0007 | アクティビティ管理 | 最終ログイン等の記録 | 🟢 P2 |  |

---

### 認証
| ID | 機能名 | 概要 | 優先度 | 備考 |
|----|--------|------|--------|------|
| Auth-0001 | ログイン | username + password による認証 | 🔴 P0 |  |
| Auth-0002 | パスワード検証 | ハッシュ照合処理 | 🔴 P0 | argon2推奨 |
| Auth-0003 | パスワード変更 | 現在パスワード検証付き変更 | 🔴 P0 |  |
| Auth-0004 | 認証失敗制御 | 試行回数制限・遅延 | 🟡 P1 |  |
| Auth-0005 | アカウントロック | 一時ロック機能 | 🟡 P1 |  |

---

### OAuth Provider
| ID | 機能名 | 概要 | 優先度 | 備考 |
|----|--------|------|--------|------|
| OAuth-0001 | クライアント登録 | client_id / secret の発行・管理 | 🔴 P0 | 内部サービス用 |
| OAuth-0002 | 認可エンドポイント | `/authorize` 認可処理 | 🔴 P0 | code発行 |
| OAuth-0003 | ログイン連携 | 未認証時にログインへリダイレクト | 🔴 P0 |  |
| OAuth-0004 | 同意画面 | scope同意UI | 🔴 P0 | 初回のみ |
| OAuth-0005 | 認可コード発行 | authorization code生成 | 🔴 P0 | 有効期限短 |
| OAuth-0006 | トークンエンドポイント | `/token` でtoken発行 | 🔴 P0 | code→token交換 |
| OAuth-0007 | アクセストークン発行 | access_token生成 | 🔴 P0 | JWT想定 |
| OAuth-0008 | リフレッシュトークン発行 | refresh_token生成 | 🔴 P0 |  |
| OAuth-0009 | トークン検証 | access_token検証API | 🔴 P0 | resource server用 |
| OAuth-0010 | トークン失効 | revoke処理 | 🟡 P1 | logout連携 |
| OAuth-0011 | スコープ管理 | scope定義・制御 | 🟡 P1 |  |
| OAuth-0012 | state管理 | CSRF防止 | 🔴 P0 | 必須 |
| OAuth-0013 | PKCE対応 | code_verifier検証 | 🟡 P1 | public client用 |
| OAuth-0014 | クライアント認証 | client_secret検証 | 🔴 P0 | confidential client |
| OAuth-0015 | OpenID対応 | id_token発行 | ⚪ P3 | OIDC拡張 |

---

### セッション
| ID | 機能名 | 概要 | 優先度 | 備考 |
|----|--------|------|--------|------|
| Session-0001 | セッション作成 | ログイン状態の保持 | 🔴 P0 | Cookie/DB |
| Session-0002 | セッション検証 | 認証状態確認 | 🔴 P0 | Middleware |
| Session-0003 | セッション失効 | ログアウト処理 | 🔴 P0 |  |
| Session-0004 | セッション期限管理 | 有効期限制御 | 🔴 P0 |  |

---

### 権限
| ID | 機能名 | 概要 | 優先度 | 備考 |
|----|--------|------|--------|------|
| Role-0001 | ロール管理 | roleの定義 | 🟡 P1 |  |
| Role-0002 | 権限管理 | permission定義 | 🟡 P1 | scopeと連携可 |
| Role-0003 | ユーザー権限付与 | role割当 | 🟡 P1 |  |
| Role-0004 | 認可チェック | APIアクセス制御 | 🔴 P0 |  |

---

## フェーズ整理

### 【Phase 0 - MVP（ユーザー管理）】

- [ ]  users
- [ ]  admin/memberロール
- [ ]  監査ログ
- [ ]  管理画面

### 【Phase 1 - OAuth】

- [ ]  authorize / token
- [ ]  JWT署名
- [ ]  oauth_clients

### 【Phase 2 - 体験向上】

- [ ]  UserInfo
- [ ]  Refresh Tokenローテーション
- [ ]  Scope管理
- [ ]  トークン失効

### 【Phase 3 - 拡張】

- [ ]  招待制登録
- [ ]  API利用制限

---

## 補足

### 優先順位の定義基準（共通ルール）
| 優先度 | 判断基準 |
| --- | --- |
| 🔴 **P0** | これがないとプロダクトが成立しない |
| 🟡 **P1** | 体験を大きく向上させる |
| 🟢 **P2** | 運用効率・継続率を改善する |
| ⚪ **P3** | 将来の拡張性・差別化 |

### 事実
- OAuth Provider実装の最小構成は  
  「authorize / token / client管理 / code / token発行」
- state と PKCE はセキュリティ的に重要
- セッションとOAuthトークンは別物（混ぜない）

### 推測（設計のコア）
- Jyogi OAuthは「認証サーバー」として動く構成になる  
  → 他サービスがそれを利用する前提設計にすると拡張しやすい

---