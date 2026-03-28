# ADR 0004 — 権限をロールテーブルの bool カラムで管理する

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-01 |
| 関連テーブル | `roles`, `user_roles` |

---

## 背景

ユーザーの権限管理の方式を検討する必要があった。  
現時点で必要な権限の種類は限定的で、管理者系・運用系の固定した権限セットが想定されている。

---

## 問題

権限管理の設計は後から変更コストが高い。  
シンプルさと将来の拡張性のバランスをどこに置くかを決める必要があった。

---

## 検討した選択肢

### A. bool カラム方式 — 採用

各権限を `roles` テーブルの bool カラムとして定義する。

```sql
CREATE TABLE roles (
    id               BIGSERIAL PRIMARY KEY,
    name             VARCHAR(50) NOT NULL UNIQUE,
    is_super_admin   BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_users BOOLEAN NOT NULL DEFAULT FALSE,
    can_revoke_tokens BOOLEAN NOT NULL DEFAULT FALSE,
    ...
);
```

- 権限の確認が `role.can_manage_users = TRUE` のような単純な参照で完結する
- JOIN が少なく、クエリがシンプルで読みやすい
- 新しい権限の追加はカラム追加のマイグレーションが必要（スキーマ変更を伴う）
- 権限の種類が増えるほどカラムが横に広がる
- 現時点の権限数（10種類程度）であれば管理可能な範囲

### B. 権限テーブル + 中間テーブル方式（RBAC）

`permissions` テーブルを別に切り、`role_permissions` で多対多に管理する。

```
roles (1) ──< role_permissions (多) >── permissions (1)
```

- 権限の追加・削除がスキーマ変更なしでデータの INSERT/DELETE のみで行える
- 動的な権限管理・カスタムロール作成など柔軟な拡張が可能
- テーブルが増え、権限確認のたびに JOIN が必要になる
- 現時点の要件に対してオーバーエンジニアリング

### C. スコープ文字列方式

権限を `"users:read users:write tokens:revoke"` のようなスペース区切りの文字列で管理する。

- JWT のスコープクレームとの親和性が高い
- 権限の確認にパース処理が必要で、型安全性が低い
- DB での検索・絞り込みが難しい

---

## 決定

**選択肢 A（bool カラム方式）を採用する。**

現時点の権限要件は固定的で種類も限られており、シンプルさを優先する。  
権限の確認ロジックが単純になることで、実装・レビュー・デバッグのコストを下げる。

現在定義している権限は以下の通り。

| カラム | 概要 |
|--------|------|
| `is_super_admin` | すべての操作が可能 |
| `can_manage_users` | ユーザーの作成・編集・削除 |
| `can_manage_roles` | ロールの作成・編集・削除 |
| `can_manage_clients` | OAuth クライアントの管理 |
| `can_manage_scopes` | スコープの管理 |
| `can_view_audit_logs` | 監査ログの閲覧 |
| `can_view_auth_events` | 認証イベントの閲覧 |
| `can_revoke_tokens` | トークンの強制失効 |
| `can_revoke_sessions` | セッションの強制失効 |
| `can_view_users` | ユーザー情報の閲覧 |

---

## 影響と制約

- 新しい権限を追加する場合はカラム追加のマイグレーションが必要になる
- 権限の種類が大幅に増えた場合（目安: 20〜30種類超）は、選択肢 B（権限テーブル方式）への移行を検討する
- `is_super_admin = TRUE` のロールはすべての権限チェックをバイパスする実装とし、個別カラムの値に関わらず全操作を許可する

---

## 参考

- 
