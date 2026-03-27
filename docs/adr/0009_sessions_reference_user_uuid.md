# ADR 0009 — sessions テーブルは user_id ではなく user_uuid を参照する

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-14 |
| 関連テーブル | `sessions`, `users` |
| 関連 ADR | [ADR 0001](./0001_db_internal_vs_public_id.md) |

---

## 背景

ADR 0001 にて `users` テーブルは内部結合用の `id`（BIGSERIAL）と外部公開用の `uuid`（UUID）を共存させる方針を決定した。  
`sessions` テーブルが `users` を参照する際、どちらのカラムを FK として使うかを検討した。

---

## 検討した選択肢

### A. `user_id`（BIGSERIAL）を参照する — 却下

- 整数型のため JOIN が高速でインデックス効率が良い
- しかし `sessions` は JWT のペイロードや API レスポンスに関わる外部境界に近いテーブルであり、内部 ID が間接的に外部へ漏れるリスクがある
- ADR 0001 の「内部 ID を外部に露出しない」方針と矛盾する

### B. `user_uuid`（UUID）を参照する — 採用

- `sessions` から `users` を参照する際に内部 ID を経由しない
- JWT のペイロード（`sub` クレーム）に `user_uuid` を使う構成と一致し、セッションから JWT 発行までの流れで内部 ID が登場しない
- ADR 0001 の方針と一貫性が保たれる
- JOIN コストは `user_id` より高いが、セッション参照の頻度・規模では許容範囲

---

## 決定

**選択肢 B（`user_uuid` を参照）を採用する。**

```sql
CREATE TABLE sessions (
    id          BIGSERIAL PRIMARY KEY,
    user_uuid   UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    ...
);
```

`sessions` から `users` への参照は `user_uuid` のみとし、内部 ID はバックエンドの内部結合にのみ使用する。

---

## 影響と制約

- `users.uuid` にインデックスが必要（UNIQUE 制約により自動で張られている）
- `sessions` から `users.id` への JOIN が必要な場合は `users.uuid` 経由で一度 `users` を引く
- この方針は `sessions` に限らず、外部境界に近いテーブルが `users` を参照する場合の原則とする

---

## 参考

- [ADR 0001 — 内部IDと公開IDの分離](./0001_db_internal_vs_public_id.md)
