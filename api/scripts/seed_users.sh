#!/usr/bin/env bash

set -e
# ^ Exit immediately if any command returns a non-zero exit status

ENV_FILE=".env.local"

if [ -f "$ENV_FILE" ]; then
# Load environment variables from .env file
    source "$ENV_FILE"
fi

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  echo >&2 "Use:"
  echo >&2 "    libpq, libpq-dev, or postgresql-client if PostgreSQL is not installed on the system"
  echo >&2 "to install it."
  exit 1
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${POSTGRES_PASSWORD}"
until psql -h ${POSTGRES_HOST} -U "${POSTGRES_USER}" -p "${POSTGRES_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1 
done

# Fixed password hash for all users
# everythinghastostartsomewhere
PASSWORD_HASH='$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8'

NUM_USERS=50

NUM_ADMINS=2

NUM_REVIEWERS=$((NUM_USERS - NUM_ADMINS))

generate_name() {
  local uuid
  local shortened_uuid

  uuid=$(uuidgen)

  shortened_uuid=$(echo "${uuid//-*/}" | cut -c1-5)

  echo "${shortened_uuid}"
}

generate_email() {
  local uuid
  local shortened_uuid

  uuid=$(uuidgen)

  shortened_uuid=$(echo "${uuid//-*/}" | cut -c1-5)

  echo "${shortened_uuid}@example.com"
}

for i in $(seq 1 $NUM_ADMINS); do
  psql -U "$POSTGRES_USER" -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -d "$POSTGRES_DB" -c "
    INSERT INTO users (name, email, password_hash, role)
    VALUES ('$(generate_name)', '$(generate_email)', '${PASSWORD_HASH}', 'Admin');
  "
done

for i in $(seq 1 $NUM_REVIEWERS); do
  psql -U "$POSTGRES_USER" -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -d "$POSTGRES_DB" -c "
    INSERT INTO users (name, email, password_hash, role)
    VALUES ('$(generate_name)', '$(generate_email)', '${PASSWORD_HASH}', 'Reviewer');
  "
done

>&2 echo "Seeded $NUM_ADMINS Admins"
>&2 echo "Seeded $NUM_REVIEWERS Reviewers"
