#!/usr/bin/env bash

set -x # Enable debugging
set -eo pipefail 
# ^ Ensures the script exits immediately if any command fails (-e) and that the exit status of a pipeline 
# is determined by the last non-zero status

ENV_FILE=".env.local"

if [ -f "$ENV_FILE" ]; then
    # Load environment variables from .env file
    source "$ENV_FILE"
fi

if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "Error: docker is not installed."
    exit 1
fi

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    echo >&2 "Use:"
    echo >&2 "    libpq, libpq-dev, or postgresql-client if PostgreSQL is not installed on the system"
    echo >&2 "to install it."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}

# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Check if a custom database name has been set, otherwise default to 'konsider'
DB_NAME="${POSTGRES_DB:=konsider}"

# Check if a custom host has been set, otherwise default to '127.0.0.1'
DB_HOST="${POSTGRES_HOST:=127.0.0.1}"

# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP}" ]]
then
    docker run \
        --name "postgres-local" \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p ${DB_HOST}:${DB_PORT}:5432 \
        -d postgres:16-alpine \
        postgres -N 100 
        # In Postgres, the default limit is typically 100 open connections, 
        # minus 3 which are reserved for superusers 
        # (putting the default limit for unprivileged users at 97 connections)
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h ${DB_HOST} -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1 
done

echo "Postgres is up on ${DB_HOST}:${DB_PORT} - running migrations"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

sqlx database create
sqlx migrate run

echo "Migrated successfully"
