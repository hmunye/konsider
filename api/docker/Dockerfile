## -------------------------------------------------------------------------------------
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef

WORKDIR /app

RUN apt-get update -y && apt-get install lld clang -y 
## -------------------------------------------------------------------------------------
FROM chef AS planner

COPY . .

# Compute a lock-like file for project
RUN cargo chef prepare --recipe-path recipe.json
## -------------------------------------------------------------------------------------
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin api
## -------------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN groupadd -r api && useradd -r -g api api-user

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends curl postgresql-client \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /app/api
COPY --from=builder /app/migrations /app/migrations
COPY .env.production /app/.env.production
COPY ./docker_entrypoint.sh /docker_entrypoint.sh

RUN chmod +x /docker_entrypoint.sh

RUN chown -R api-user:api /app /docker_entrypoint.sh

USER api-user

ENV ENVIRONMENT=production

ENTRYPOINT ["/docker_entrypoint.sh"]
