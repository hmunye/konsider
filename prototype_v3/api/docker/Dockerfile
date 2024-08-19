# Using .dockerignore file, splitting into builder and runtime stages, and using
# bare operating system "debain:bullseye-slim" reduces size of image to ~86MB

# Builder Stage
FROM rust:1.80.1 AS builder

WORKDIR /app

RUN apt-get update -y && apt-get install lld clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

# Runtime Stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app
# Install OpenSSL - dynamically linked by some of the dependencies
# Install ca-certificates - needed to verify TLS certificates 
# when establishing HTTPS connections

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api api

COPY .env.production .env.production

ENV ENVIRONMENT=production

ENTRYPOINT ["./api"]
