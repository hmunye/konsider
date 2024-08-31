# set -x
# set -eo pipefail

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

>&2 echo "Redis is up and ready"
