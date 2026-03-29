FROM rust:1.77-slim AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY db/ db/
COPY migration/ migration/

RUN cargo build --release -p migration

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/migration ./migration
COPY migration/migrations/ ./migrations/

CMD ["./migration"]