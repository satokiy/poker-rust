FROM rust:1.88.0 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/poker-rust /app/poker-rust

RUN chmod +x /app/poker-rust

EXPOSE 3000

CMD ["./poker-rust"]
