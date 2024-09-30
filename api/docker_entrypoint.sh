#!/usr/bin/env bash

set -eo pipefail 
# ^ Ensures the script exits immediately if any command fails (-e) and that the exit status of a pipeline 
# is determined by the last non-zero status

ENV_FILE="/app/.env.production"

if [ -f "$ENV_FILE" ]; then
    # Load environment variables from .env file
    source "$ENV_FILE"
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD=${POSTGRES_PASSWORD}
until psql -h postgres -U ${POSTGRES_USER} -p ${POSTGRES_PORT} -d postgres -c '\q' > /dev/null 2>&1; do
    echo >&2 "Postgres is still unavailable - sleeping..."
    sleep 1
done

# Run migrations from SQL files
for f in /app/migrations/*.sql; do
    echo "Running migration: $f"
    if ! psql -h postgres -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "${POSTGRES_DB}" -f "$f"; then
        echo >&2 "Failed to run migration: $f"
        continue
    fi
done

unset PGPASSWORD

exec /app/api
