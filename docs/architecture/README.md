# Architecture

## リポジトリ構成

![モノレポ構成](./monorepo.svg)

## クレート概要

### `api/`
Axum を使った HTTP API サーバー。  
ルーター・ハンドラー・ミドルウェアを管理する。`db` クレートに依存する。

- フレームワーク: [Axum](https://github.com/tokio-rs/axum)
- ランタイム: Tokio
- 主な責務: ルーティング、リクエスト/レスポンス処理、認証ミドルウェア

### `migration/`
sqlx-cli によるデータベースマイグレーション管理。  
`migrations/*.sql` ファイルを順番に適用する。

- ツール: [sqlx-cli](https://github.com/launchbadge/sqlx)
- 主な責務: スキーマバージョン管理、マイグレーションの実行・ロールバック

### `db/`
PostgreSQL への接続プールとクエリを提供するライブラリクレート。  
`api` と `migration` から共通して使用される。

- ドライバ: sqlx + PostgreSQL
- 主な責務: コネクションプール、モデル定義、クエリ関数

### `web/`
Next.js による管理画面フロントエンド。  
`api` の REST エンドポイントを呼び出す。

- フレームワーク: Next.js (App Router)
- 言語: TypeScript
- 主な責務: 管理 UI、認証フロー、データ表示・操作

## 依存関係

```
api  ──depends on──▶  db
web  ──calls──────▶  api  (HTTP)
migration  ──uses──▶  db
```

## 開発環境の起動

```bash
# PostgreSQL 起動
docker compose up -d

# マイグレーション実行
cargo run -p migration

# API サーバー起動
cargo run -p api

# 管理画面起動
cd web && npm run dev
```

## 環境変数

| 変数名 | 説明 | 例 |
|---|---|---|
| `DATABASE_URL` | PostgreSQL 接続文字列 | `postgres://postgres:password@localhost:5432/jyogi_oauth` |
| `LISTEN_ADDR` | API サーバーのバインドアドレス | `0.0.0.0:8080` |
