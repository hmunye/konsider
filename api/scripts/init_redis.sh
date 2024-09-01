# set -x
# set -eo pipefail

ENV_FILE=".env.local"

if [ -f "$ENV_FILE" ]; then
# Load environment variables from .env file
    source "$ENV_FILE"
fi

if ! [ -x "$(command -v redis-cli)" ]; then
  echo >&2 "Error: redis is not installed."
  exit 1
fi

# Check if a custom port has been set, otherwise default to '6379'
REDIS_PORT="${REDIS_PORT:=6379}"

# Check if a custom host has been set, otherwise default to '127.0.0.1'
REDIS_HOST="${REDIS_HOST:=127.0.0.1}"

RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')

# If a redis container is running, print instructions to kill it and exit
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "There is a redis container already running, kill it with"
  echo >&2 "    docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

# Launch Redis using Docker
docker run \
    --name "redis" \
    -p "127.0.0.1:6379:6379" \
    -d \
    redis

echo >&2 "Redis is up and ready"
