# Stage 1: Build the binary
FROM rust:1.91.0-slim-bookworm AS builder
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo install sqlx-cli --no-default-features --features postgres

COPY ./migrations ./migrations
COPY ./src ./src

COPY ./.sqlx ./.sqlx
ENV SQLX_OFFLINE=true

RUN cargo build

# Stage 2: Minimal Runtime Image
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/debug/backend .
COPY --from=builder /app/migrations ./migrations

ENV DATABASE_URL postgres://postgres:postgres@db:5432/solwatch
EXPOSE 5000

ENTRYPOINT ["./backend"]
