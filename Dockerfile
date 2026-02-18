# ===============================
# Rust base (cargo-chef)
# ===============================
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
RUN apt update && apt install -y lld clang

# ===============================
# Planner
# ===============================
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ===============================
# Rust builder
# ===============================
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin rpg_stage

# ===============================
# Frontend builder (Node)
# ===============================
FROM node:20-alpine AS frontend_builder
WORKDIR /client

COPY client/ .

RUN npm install
RUN npm run build

# ===============================
# Runtime
# ===============================
FROM rust:latest AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Rust binary
COPY --from=builder /app/target/release/rpg_stage rpg_stage

# 前端 dist
COPY --from=frontend_builder /client/dist ./client/dist

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./rpg_stage"]
