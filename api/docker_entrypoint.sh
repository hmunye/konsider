#!/usr/bin/env bash

set -e
# ^ Exit immediately if any command returns a non-zero exit status

if [ -f /app/.env.production ]; then
  export $(grep -v '^#' /app/.env.production | xargs)
fi

export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:${POSTGRES_PORT}/${POSTGRES_DB}

export PGPASSWORD=${POSTGRES_PASSWORD}

until psql -h postgres -U ${POSTGRES_USER} -p ${POSTGRES_PORT} -d postgres -c '\q' > /dev/null 2>&1; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

sqlx database create
sqlx migrate run

exec /app/api
