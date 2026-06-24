# syntax=docker/dockerfile:1

# Build from the repository root:
#   docker build -f api/Dockerfile -t worklogger-api .
#
# Rebuild, replace the running container, and prune the old image:
#   export DATABASE_URL='postgres://USER:PASSWORD@db-host:5432/worklog'
#   ./scripts/redeploy-worklogger-api.sh
#
# Run against an external PostgreSQL service (migrations are not applied by this image):
#   docker run --rm -p 3000:3000 \
#     -e DATABASE_URL='postgres://USER:PASSWORD@db-host:5432/worklog' \
#     worklogger-api

FROM rust:bookworm AS builder

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml
COPY core/Cargo.toml core/Cargo.toml
COPY common/Cargo.toml common/Cargo.toml
COPY use_cases/Cargo.toml use_cases/Cargo.toml
COPY infrastructure/Cargo.toml infrastructure/Cargo.toml
COPY tui/Cargo.toml tui/Cargo.toml
COPY sdk/Cargo.toml sdk/Cargo.toml
COPY admin/Cargo.toml admin/Cargo.toml

# Workspace lists `tui`, `sdk`, and `admin`; stub them so only `api` is built.
RUN mkdir -p tui/src admin/src sdk/src \
    && printf 'fn main() {}\n' > tui/src/main.rs \
    && printf 'fn main() {}\n' > admin/src/main.rs \
    && printf '// stub\n' > sdk/src/lib.rs

COPY .sqlx .sqlx
COPY infrastructure/.sqlx infrastructure/.sqlx

COPY api/src api/src
COPY core/src core/src
COPY common/src common/src
COPY use_cases/src use_cases/src
COPY infrastructure/src infrastructure/src

ENV SQLX_OFFLINE=true
RUN cargo build --release -p api

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --create-home --shell /usr/sbin/nologin appuser

COPY --from=builder /app/target/release/api /usr/local/bin/api

USER appuser

ENV HOST=0.0.0.0
ENV PORT=3000
EXPOSE 3000

CMD ["api"]
