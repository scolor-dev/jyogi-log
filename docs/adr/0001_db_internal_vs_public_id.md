# ADR 0001 — 内部IDと公開IDの分離（`id` vs `uuid`）

| 項目 | 内容 |
|------|------|
| ステータス | 決定済み |
| 決定日 | 2026-03-01 |
| 関連テーブル | `users`, `sessions` |

---

## 背景

`users` テーブルの主キーとして `BIGSERIAL`（連番整数）を採用した。  
連番IDをそのまま外部に公開すると以下のリスクがある。

- **列挙攻撃**: `id=1`, `id=2` ... と順番にアクセスすることでユーザー総数や存在確認ができてしまう
- **情報漏洩**: 連番からユーザー登録順・規模が推測できる
- **結合の硬直化**: 外部サービスやフロントエンドが内部IDに依存すると、DB再構築時に影響範囲が広がる

一方、UUIDのみをPKにする設計も検討したが、JOINが多いシステムでは整数PKの方がインデックス効率が高く、INSERTのパフォーマンスも安定する。

---

## 検討した選択肢

### A. 連番ID（BIGSERIAL）のみ — 却下

- シンプルで実装コストが低い
- 外部公開すると列挙攻撃・情報漏洩のリスクがある
- APIレスポンスやURLに露出させられない

### B. UUIDのみ — 却下

- 外部公開しても比較的安全
- PKがUUIDになるとインデックスサイズが大きくなり、大量INSERTでページ分割が頻発する
- JOIN・ソートのコストが整数PKより高い

### C. 連番ID（内部用）+ UUID（外部公開用）の併用 — 採用

- 内部結合は `id`（BIGSERIAL）を使い、パフォーマンスを確保する
- 外部に露出するIDは `uuid`（UUID v4）のみに限定し、セキュリティリスクを排除する
- `uuid` には `UNIQUE` 制約と `DEFAULT gen_random_uuid()` を設定し、アプリ側での生成を不要にする

---

## 決定

**選択肢 C を採用する。**

`users` テーブルに `id`（BIGSERIAL, PK）と `uuid`（UUID, UNIQUE）を共存させる。

```sql
CREATE TABLE users (
    id   BIGSERIAL PRIMARY KEY,
    uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    ...
);
```

外部との境界（APIレスポンス・URL・他テーブルからの外部参照のうち外部公開を伴うもの）では `uuid` のみを使用する。  
`sessions` テーブルが `users.uuid` を外部参照しているのはこの方針に基づく。

---

## 影響と制約

- APIレスポンスでは `id` を返さず、`uuid` を `id` フィールドとして返すか、`uuid` フィールドとして明示する
- フロントエンドはユーザー識別に `uuid` を使う
- 内部サービス間通信（バックエンド同士）では `id` を使ってよい
- 新たに `users` を参照するテーブルを作る際は、外部公開を伴うかどうかで参照カラムを使い分ける

---

## 参考

- [PostgreSQL における UUID と bigint：スケールする ID の選び方](https://appmaster.io/ja/blog/postgresql-no-uuid-vs-bigint-sukeraburuna-id-noxuan-bifang)
- qiita — [主キーは AUTO_INCREMENT と UUID のどちらを選ぶべきか](https://qiita.com/tonbi_attack/items/caeadbb8ce8c701d316b)
- PostgreSQL ドキュメント — [UUID型](https://www.postgresql.org/docs/current/datatype-uuid.html)
- PostgreSQL ドキュメント — [pgcrypto — gen_random_uuid()](https://www.postgresql.org/docs/current/pgcrypto.html)
