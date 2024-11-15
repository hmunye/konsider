#!/usr/bin/env bash

# set -x # Enable debugging
set -eo pipefail 
# ^ Ensures the script exits immediately if any command fails (-e) and that 
# the exit status of a pipeline is determined by the last non-zero status

if ! [ -x "$(command -v docker)" ]; then
    echo >&2 "Error: docker is not installed"
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed"
    echo >&2 "Use:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it"
    exit 1
fi

TOML_FILE="$PWD/config/local.toml"

if [ ! -f "$TOML_FILE" ]; then
    echo >&2 "local config file not found: $TOML_FILE"
    exit 1
fi

# Function to extract value from TOML file
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

# Check if a custom superuser has been set, otherwise default to 'postgres'
# Would be passed as an environment variable
SUPERUSER="${SUPERUSER:=postgres}"

# Check if a custom superuser password has been set, otherwise default to 'password'
# Would be passed as an environment variable
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"

# Reads values from the `local.toml` configuration file
export DB_USER=$(get_toml_value "user")

export DB_USER_PWD=$(get_toml_value "password")

export DB_NAME=$(get_toml_value "database")

# This is the port the host machine will use to connect to the container
export DB_PORT=$(get_toml_value "db_port")

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
    RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=postgres' --format '{{.ID}}')

    # If a Postgres container is running, output instructions to kill it and exit
    if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
        echo >&2 "Postgres container already running, kill it with"
        echo >&2 "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
        exit 1
    fi

    CONTAINER_NAME="postgres_$(date '+%s')"

    docker run \
        --env POSTGRES_USER=${SUPERUSER} \
        --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
        --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
        --health-interval=1s \
        --health-timeout=5s \
        --health-retries=5 \
        --publish "${DB_PORT}":5432 \
        --detach \
        --name "${CONTAINER_NAME}" \
        postgres:alpine -N 1000
        # In Postgres, the default limit is typically 100 open connections, 
        # minus 3 which are reserved for superusers 
        # (putting the default limit for unprivileged users at 97 connections)
        # Set to 1000 for testing
      
    until [ \
        "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
        "healthy" \
    ]; do     
        >&2 echo "Postgres is still unavailable - sleeping..."
        sleep 1 
    done
  
    CREATE_QUERY="CREATE USER ${DB_USER} WITH ENCRYPTED PASSWORD '${DB_USER_PWD}';"
    docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

    GRANT_QUERY="ALTER USER ${DB_USER} CREATEDB;"
    docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
fi

>&2 echo "Postgres is now up on port ${DB_PORT} - running migrations..."

export DATABASE_URL=postgres://${DB_USER}:${DB_USER_PWD}@localhost:${DB_PORT}/${DB_NAME}

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated"
