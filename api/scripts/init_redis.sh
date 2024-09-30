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

# Check if a custom host has been set, otherwise default to '127.0.0.1'
REDIS_HOST="${REDIS_HOST:=127.0.0.1}"

# Check if a custom port has been set, otherwise default to '6379'
REDIS_PORT="${REDIS_PORT:=6379}"

# Check if a password has been set, otherwise default to no password
REDIS_PASSWORD="${REDIS_PASSWORD:=}"

RUNNING_CONTAINER=$(docker ps --filter 'name=redis-local' --format '{{.ID}}')

# If a redis local container is running, print instructions to kill it and exit
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "There is a redis container already running, kill it with"
  echo >&2 "    docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

# Launch Redis using Docker
docker run \
    --name "redis-local" \
    -p ${REDIS_HOST}:${REDIS_PORT}:6379 \
    -d \
    redis:7-alpine \
    redis-server --requirepass "$REDIS_PASSWORD"

echo "Redis is up and ready"
