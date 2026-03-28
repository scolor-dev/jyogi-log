# ADR 0010 — トークンの保存場所（AccessToken: メモリ / RefreshToken: httpOnly Cookie）

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-14 |
| 関連 | Frontend / 認証フロー |
| 関連 ADR | [ADR 0002](./0002_session_management.md) |

---

## 背景

フロントエンド（React SPA）でトークンをどこに保存するかは、XSS・CSRF それぞれの攻撃面に直接影響する。  
AccessToken と RefreshToken は性質が異なるため、それぞれ独立して検討した。

---

## 検討 1 — AccessToken の保存場所

### 選択肢

| 保存場所 | XSS 耐性 | CSRF 耐性 | ページリロード後 |
|---------|---------|---------|--------------|
| メモリ（JS 変数） | 高 | 高（Cookie でないため自動送信されない） | 消える |
| localStorage | 低（XSS で容易に窃取される） | 高 | 残る |
| sessionStorage | 低（XSS で窃取可能） | 高 | タブを閉じると消える |
| httpOnly Cookie | 高（JS からアクセス不可） | 低（自動送信される） | 残る |

### 決定: メモリ（JS 変数）を採用

- AccessToken は有効期限が短い（15分程度）ため、ページリロードで消えても RefreshToken Cookie で再取得できる
- localStorage / sessionStorage は XSS で容易に窃取されるため却下
- httpOnly Cookie は CSRF 対策が別途必要になるため却下
- メモリに置くことで XSS・CSRF 両面で最も安全な保存場所となる

---

## 検討 2 — RefreshToken の保存場所

### 選択肢

| 保存場所 | XSS 耐性 | CSRF 耐性 | 備考 |
|---------|---------|---------|------|
| メモリ（JS 変数） | 高 | 高 | ページリロードでログアウト扱いになる |
| localStorage | 低 | 高 | XSS で窃取されると長期間悪用される |
| httpOnly Cookie | 高（JS からアクセス不可） | 要対策 | リロード後も維持される |

### 決定: httpOnly Cookie を採用

- RefreshToken は有効期限が長いため、メモリに置くとリロードのたびに再ログインが必要になり UX が壊れる
- localStorage は XSS で窃取された場合に長期間悪用されるリスクがあり、有効期限の長い RefreshToken との相性が最悪のため却下
- httpOnly Cookie は JS からアクセスできないため XSS で窃取できない
- CSRF については `POST /auth/refresh` に `SameSite=Strict` を設定することで対策する

---

## 決定まとめ

| トークン | 保存場所 | 送信方法 |
|---------|---------|---------|
| AccessToken | メモリ（React state / store） | `Authorization: Bearer {token}` ヘッダー |
| RefreshToken | httpOnly Cookie | Cookie として自動送信 |

実装は以下の通り。

```rust
// RefreshToken を httpOnly Cookie にセットし、AccessToken はボディで返す
let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
    .http_only(true)
    .path("/")
    .build();

Ok((
    StatusCode::OK,
    jar.add(refresh_cookie),
    Json(LoginResponse { access_token }),
))
```

```
ログイン・リフレッシュ成功
  → AccessToken: レスポンスボディで受け取り、メモリ（store）に保持
  → RefreshToken: サーバーが Set-Cookie で httpOnly Cookie にセット

APIリクエスト
  → Authorization: Bearer {AccessToken} をヘッダーに付与

ページリロード
  → メモリの AccessToken は消える
  → httpOnly Cookie の RefreshToken で POST /auth/refresh を叩き、新 AccessToken をメモリに再取得

ログアウト
  → メモリの AccessToken を破棄
  → POST /auth/logout でサーバー側セッションを失効
  → サーバーが Set-Cookie で RefreshToken Cookie を削除（空・過去日時）
```

---

## 影響と制約

- ページリロード時に必ず `/auth/refresh` が走るため、初期表示に若干のレイテンシが発生する。ローディング状態の適切な UI 処理が必要
- RefreshToken Cookie には `HttpOnly`, `Secure`, `SameSite=Strict` を設定する
- クロスオリジン構成（フロントエンドとバックエンドのドメインが異なる場合）では `SameSite=Strict` が機能しないため、CORS 設定と合わせて慎重に検討する

---

## 参考

- [OWASP — HTML5 Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html)
- [OWASP — Cross-Site Request Forgery Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html)
- [Auth0 Blog — Token Storage](https://auth0.com/docs/secure/security-guidance/data-security/token-storage)
