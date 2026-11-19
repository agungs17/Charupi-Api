# Multi-stage Dockerfile for Charupi-Api
# Stage 1: Chef to compute recipe
FROM rust:1.85-slim-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef

# Stage 2: Planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin charupi-api

# Stage 4: Minimal Runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libsqlite3-0 \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/charupi-api /usr/local/bin/charupi-api
COPY .env.example /app/.env

# Create data directory for SQLite
RUN mkdir -p /app/data

ENV HOST=0.0.0.0
ENV PORT=8080
ENV ENVIRONMENT=production
ENV DATABASE_URL=sqlite:///app/data/charupi.db?mode=rwc

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["charupi-api"]
