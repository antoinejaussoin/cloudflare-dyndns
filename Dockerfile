# Multi-stage build for cloudflare-dyndns (Rust)
# Context: repository root (contains Cargo.toml / Cargo.lock)

FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src
# Touch so cargo rebuilds the binary after replacing the stub
RUN touch src/main.rs && cargo build --release

FROM alpine:3.21

RUN apk add --no-cache ca-certificates \
    && adduser -D -H -u 1000 dyndns

COPY --from=builder /app/target/release/cloudflare-dyndns /usr/local/bin/cloudflare-dyndns

USER dyndns

CMD ["cloudflare-dyndns"]
