default:
    just --list

# DB起動（マイグレーションも自動実行）
db:
    docker compose -f migration/docker-compose.yml up -d --wait db
    docker compose -f migration/docker-compose.yml run --rm migrate

# DB停止
db-down:
    docker compose -f migration/docker-compose.yml down

# API起動
api:
    cd api && cargo run

# DB起動 + API起動
dev:
    just db
    just api