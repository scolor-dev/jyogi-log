FROM rust:1.77-slim
 
RUN cargo install sqlx-cli \
    --no-default-features \
    --features postgres
 
WORKDIR /app
COPY migration/migrations/ ./migrations/
 
CMD ["sqlx", "migrate", "run", "--source", "./migrations"]
 