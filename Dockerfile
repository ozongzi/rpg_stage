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
# Frontend builder (Deno)
# ===============================
FROM denoland/deno:alpine AS frontend_builder
WORKDIR /client

# 只复制前端目录，避免破坏缓存
COPY client/ .

# 如果有依赖缓存
RUN deno install

# 执行构建
RUN deno run build

# ===============================
# Runtime
# ===============================
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 拷贝 Rust binary
COPY --from=builder /app/target/release/rpg_stage rpg_stage

# 拷贝前端 dist
COPY --from=frontend_builder /client/dist ./client/dist

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./rpg_stage"]
