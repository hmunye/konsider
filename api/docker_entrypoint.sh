#!/usr/bin/env bash

set -e
# ^ Exit immediately if any command returns a non-zero exit status

ENV_FILE="/app/.env.production"

if [ -f "$ENV_FILE" ]; then
# Load environment variables from .env file
    source "$ENV_FILE"
fi

export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:${POSTGRES_PORT}/${POSTGRES_DB}
export PGPASSWORD=${POSTGRES_PASSWORD}

until psql -h postgres -U ${POSTGRES_USER} -p ${POSTGRES_PORT} -d postgres -c '\q' > /dev/null 2>&1; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

# Run migrations from SQL files
for f in /app/migrations/*.sql; do
    >&2 echo "Running migration: $f"
    if ! psql -h postgres -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "${POSTGRES_DB}" -f "$f"; then
        >&2 echo "Failed to run migration: $f"
        continue
    fi
done

# unset PGPASSWORD

exec /app/api
