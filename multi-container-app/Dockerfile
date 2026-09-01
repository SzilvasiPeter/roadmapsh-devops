# syntax=docker/dockerfile:1

FROM rust:1.96-slim AS builder

WORKDIR /app

# Build and cache all dependencies (registry + compiled artifacts) so they
# persist across builds on the same host and are not recompiled each time.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src

# Recompile just the app crate (deps come from the shared cache mount).
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    touch src/main.rs \
    && cargo build --release \
    && cp /app/target/release/multi-container-app /app/multi-container-app

FROM debian:trixie-slim

COPY --from=builder /app/multi-container-app /usr/local/bin/multi-container-app

EXPOSE 3000

CMD ["multi-container-app"]
