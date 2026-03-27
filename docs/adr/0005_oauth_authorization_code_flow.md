# ADR 0005 — OAuth 2.0 フローの選択（認可コードフロー）

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-01 |
| 関連テーブル | `oauth_clients`, `oauth_authorization_codes`, `oauth_client_redirect_uris` |

---

## 背景

OAuth 2.0 の実装にあたり、どの認可フローを採用するかを検討した。  
RFC 6749 では以下の 4 つのフローが定義されている。

参考: [OAuth 2.0 全フローの図解と動画 — @TakahikoKawasaki (Qiita)](https://qiita.com/TakahikoKawasaki/items/200951e5b5929f840a1f)

---

## 検討した選択肢

### A. 認可コードフロー（Authorization Code Grant）— 採用

- 認可エンドポイントに認可リクエストを投げ、短命の認可コードを受け取る
- その認可コードをトークンエンドポイントでアクセストークンと交換する（2ステップ）
- アクセストークンがブラウザの履歴や URL に露出しない
- PKCE（RFC 7636）と組み合わせることで、public クライアント（SPA・モバイル）でも安全に使える
- リフレッシュトークンの発行が可能
- 現在 OAuth 2.0 で最も推奨されているフロー

### B. インプリシットフロー（Implicit Grant）— 却下

- 認可エンドポイントから直接アクセストークンを受け取るフロー
- SPA 向けに設計されたが、アクセストークンが URL フラグメントに露出するリスクがある
- リフレッシュトークンが発行されない
- PKCE 付き認可コードフローの普及により、現在は非推奨（RFC 9700）

### C. リソースオーナー・パスワード・クレデンシャルズフロー（ROPC）— 却下

- クライアントがユーザーの ID とパスワードを直接受け取り、トークンエンドポイントへ渡すフロー
- OAuth の設計思想（クライアントにパスワードを渡さない）に反する
- クライアントアプリケーションを完全に信頼できる場合のみ使用が許容されるが、現構成では該当しない
- 現在は非推奨

### D. クライアント・クレデンシャルズフロー（Client Credentials Grant）— 用途が異なる

- ユーザー認証を行わず、クライアントアプリケーション自体の認証のみでトークンを取得するフロー
- M2M（Machine-to-Machine）通信向けであり、ユーザーが関与しない
- 本システムでは M2M 用途として `client_tokens` テーブルで別途管理する（ユーザー向け認証フローとは別物）

---

## 決定

**選択肢 A（認可コードフロー + PKCE）を採用する。**

認可コードを短命・使い捨てとして扱い、トークンとの交換はサーバー側で行う。  
public クライアント（SPA）に対しては PKCE を必須とし、code_challenge / code_verifier で認可コード横取り攻撃を防ぐ。

```
1. クライアントが認可エンドポイントへリダイレクト
   GET /oauth/authorize
     ?response_type=code
     &client_id={client_id}
     &redirect_uri={redirect_uri}
     &scope={scope}
     &state={state}
     &code_challenge={challenge}       // PKCE
     &code_challenge_method=S256       // PKCE

2. ユーザー認証・認可後、認可コードをリダイレクトで返す
   302 Found
   Location: {redirect_uri}?code={code}&state={state}
   → code_hash を oauth_authorization_codes に INSERT（短命・使い捨て）

3. クライアントがトークンエンドポイントで認可コードを交換
   POST /oauth/token
     grant_type=authorization_code
     &code={code}
     &redirect_uri={redirect_uri}
     &code_verifier={verifier}         // PKCE
   → oauth_authorization_codes の code_hash を照合
   → consumed_at を記録（再利用不可）
   → AccessToken（JWT）と RefreshToken を発行
```

`oauth_authorization_codes` テーブルは `consumed_at` で使用済みを管理し、部分インデックス（`WHERE consumed_at IS NULL`）により未使用コードの重複を防ぐ。

---

## 影響と制約

- 認可コードは短命（数分程度）かつ使い捨てとして運用する
- `state` パラメーターを必須として CSRF 攻撃を防ぐ
- public クライアントに対して PKCE（S256）を必須とする
- `redirect_uri` は `oauth_client_redirect_uris` のホワイトリストと完全一致で検証する
- 期限切れ・使用済みの `oauth_authorization_codes` レコードは定期的にクリーンアップする

---

## 参考

- [OAuth 2.0 全フローの図解と動画 — @TakahikoKawasaki (Qiita)](https://qiita.com/TakahikoKawasaki/items/200951e5b5929f840a1f)
