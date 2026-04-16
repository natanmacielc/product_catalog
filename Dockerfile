FROM rust:1.88 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/product_catalog /app/product_catalog
COPY .env /app/.env

EXPOSE 3000

CMD ["/app/product_catalog"]