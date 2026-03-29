# Architecture

## リポジトリ構成

```svg
<svg width="100%" viewBox="0 0 680 220" xmlns="http://www.w3.org/2000/svg">
  <rect x="215" y="20" width="250" height="44" rx="10" fill="#F1EFE8" stroke="#5F5E5A" stroke-width="0.5"/>
  <text font-family="sans-serif" font-size="14" font-weight="500" fill="#2C2C2A" x="340" y="42" text-anchor="middle" dominant-baseline="central">Jyogi-OAuth/</text>

  <line x1="340" y1="64" x2="340" y2="90" stroke="#B4B2A9" stroke-width="0.5"/>
  <line x1="90"  y1="90" x2="590" y2="90" stroke="#B4B2A9" stroke-width="0.5"/>
  <line x1="90"  y1="90" x2="90"  y2="110" stroke="#B4B2A9" stroke-width="0.5"/>
  <line x1="250" y1="90" x2="250" y2="110" stroke="#B4B2A9" stroke-width="0.5"/>
  <line x1="430" y1="90" x2="430" y2="110" stroke="#B4B2A9" stroke-width="0.5"/>
  <line x1="590" y1="90" x2="590" y2="110" stroke="#B4B2A9" stroke-width="0.5"/>

  <!-- api/ -->
  <rect x="30" y="110" width="120" height="64" rx="10" fill="#EEEDFE" stroke="#534AB7" stroke-width="0.5"/>
  <text font-family="sans-serif" font-size="14" font-weight="500" fill="#3C3489" x="90" y="134" text-anchor="middle" dominant-baseline="central">api/</text>
  <text font-family="sans-serif" font-size="12" fill="#534AB7" x="90" y="156" text-anchor="middle" dominant-baseline="central">Axum HTTP server</text>

  <!-- migration/ -->
  <rect x="190" y="110" width="120" height="64" rx="10" fill="#E1F5EE" stroke="#0F6E56" stroke-width="0.5"/>
  <text font-family="sans-serif" font-size="14" font-weight="500" fill="#085041" x="250" y="134" text-anchor="middle" dominant-baseline="central">migration/</text>
  <text font-family="sans-serif" font-size="12" fill="#0F6E56" x="250" y="156" text-anchor="middle" dominant-baseline="central">sqlx migrations</text>

  <!-- db/ -->
  <rect x="370" y="110" width="120" height="64" rx="10" fill="#FAECE7" stroke="#993C1D" stroke-width="0.5"/>
  <text font-family="sans-serif" font-size="14" font-weight="500" fill="#712B13" x="430" y="134" text-anchor="middle" dominant-baseline="central">db/</text>
  <text font-family="sans-serif" font-size="12" fill="#993C1D" x="430" y="156" text-anchor="middle" dominant-baseline="central">PostgreSQL</text>

  <!-- web/ -->
  <rect x="530" y="110" width="120" height="64" rx="10" fill="#E6F1FB" stroke="#185FA5" stroke-width="0.5"/>
  <text font-family="sans-serif" font-size="14" font-weight="500" fill="#0C447C" x="590" y="134" text-anchor="middle" dominant-baseline="central">web/</text>
  <text font-family="sans-serif" font-size="12" fill="#185FA5" x="590" y="156" text-anchor="middle" dominant-baseline="central">Next.js</text>
</svg>
```

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