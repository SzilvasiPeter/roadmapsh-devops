FROM rust:1.96-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release \
    && cp /app/target/release/multi-container-app /app/multi-container-app

FROM debian:trixie-slim

COPY --from=builder /app/multi-container-app /usr/local/bin/multi-container-app

EXPOSE 3000

CMD ["multi-container-app"]
