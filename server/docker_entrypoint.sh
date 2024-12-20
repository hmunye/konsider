#!/usr/bin/env bash

# set -x # Enable debugging
set -eo pipefail 
# ^ Ensures the script exits immediately if any command fails (-e) and that the exit status of a pipeline 
# is determined by the last non-zero status

TOML_FILE="/app/config/production.toml"

if [ ! -f "$TOML_FILE" ]; then
    echo >&2 "production config file not found: $TOML_FILE"
    exit 1
fi

# Function to extract value from TOML
get_toml_value() {
    local key="$1"

    local line=$(grep -E "^\s*${key}\s*=" "$TOML_FILE")

    if [[ -z "$line" ]]; then
        echo >&2 "key '$key' not found in $TOML_FILE"
        return 1
    fi

    local value=$(echo "$line" | cut -d '=' -f 2 | tr -d ' "')

    echo "$value"
}

POSTGRES_USER=$(get_toml_value "user")
POSTGRES_PASSWORD=$(get_toml_value "password")
POSTGRES_DB=$(get_toml_value "database")
POSTGRES_HOST=$(get_toml_value "db_host")
POSTGRES_PORT=$(get_toml_value "db_port")

export PGPASSWORD="$POSTGRES_PASSWORD"
until psql -h "$POSTGRES_HOST" -U "$POSTGRES_USER" -p "$POSTGRES_PORT" -d "$POSTGRES_DB" -c '\q' > /dev/null 2>&1; do
    echo >&2 "Postgres is still unavailable - sleeping..."
    sleep 1
done

export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}?sslmode=require"

sqlx migrate run

exec /app/k6r
