# 06 **📁 ディレクトリ構成図**

# ディレクトリ構成図

## （部内OAuth基盤 / OIDC Provider版）

---

# 0️⃣ 設計前提

| 項目 | 内容 |
| --- | --- |
| リポジトリ構成 | Monorepo |
| アーキテクチャ | Modular Monolith |
| デプロイ単位 | Web：1単位, API：1単位, DB：1単位 |
| 言語 | Rust + TS |
| MVP方針 | P0必須 |

# 1️⃣ 全体構成（Monorepo）

```
root/
├── apps/
│   ├── api/            # Rust
│   └── web/            # React
├── infra/
│   ├── compose.yml     # postgres + api + web(dev)
│   └── migrations/     # sqlx migrations
├── packages/
│   └── contracts/      # OpenAPI/Schema（任意）
└── README.md
```

---

# 2️⃣ バックエンド構成（api）

```
apps/api/
├── Cargo.toml
```

---

# 3️⃣ DBマイグレーション構成

```
migrations/
├── 0001_create_users.sql
```

---

# 4️⃣ インフラ構成

```
infra/
├── docker/
│   ├── Dockerfile.auth
│   └── docker-compose.yml
├── ci/
│   └── github-actions.yml
```

---

# 5️⃣ テスト構成

```
tests/
├── unit/
├── integration/
└── fixtures/
```

---

# 設計思想まとめ

- OAuth/OIDCエンドポイントと管理APIを責務分離
- ドメイン層とインフラ層を明確分離
- users.uuid を外部公開IDとして統一
- JWT署名鍵管理を専用モジュール化