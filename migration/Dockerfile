FROM rust:1.94-slim

RUN cargo install sqlx-cli \
    --no-default-features \
    --features postgres \
    --locked

WORKDIR /app
COPY migrations/ ./migrations/

CMD ["sqlx", "migrate", "run", "--source", "./migrations"]