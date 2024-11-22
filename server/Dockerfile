FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef

WORKDIR /app

RUN apt-get update -y && apt-get install lld clang -y 
#------------------------------------------------------------------------------
FROM chef AS planner

COPY . .

# Compute a lock-like file for project
RUN cargo chef prepare --recipe-path recipe.json
#------------------------------------------------------------------------------
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin k6r
#------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN groupadd -r k6r && useradd -r -g k6r k6r-user

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends curl postgresql-client \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/k6r /app/k6r
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/config/production.toml /app/config/production.toml
COPY ./docker_entrypoint.sh /docker_entrypoint.sh

RUN chmod +x /docker_entrypoint.sh

RUN chown -R k6r-user:k6r /app /docker_entrypoint.sh

USER k6r-user

ENV ENVIRONMENT=production

ENTRYPOINT ["/docker_entrypoint.sh"]
