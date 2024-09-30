#!/usr/bin/env bash

set -eo pipefail 
# ^ Ensures the script exits immediately if any command fails (-e) and that the exit status of a pipeline 
# is determined by the last non-zero status

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
    echo >&2 "Postgres is still unavailable - sleeping"
    sleep 1 
done

# Fixed password hash for all users (everythinghastostartsomewhere)
PASSWORD_HASH='$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8'

NUM_USERS_EXPECTED=50
NUM_ADMINS_EXPECTED=2
NUM_REVIEWERS_EXPECTED=$((NUM_USERS_EXPECTED - NUM_ADMINS_EXPECTED))

NUM_ADMINS_ACTUAL=0
NUM_REVIEWERS_ACTUAL=0

generate_details() {
    name=$(sed "$(jot -r 1 1 2047)q;d" ./scripts/names.txt | sed -e 's/[^a-zA-Z]//g')
    echo "${name}"
    echo "${name}" | tr '[:upper:]' '[:lower:]' | sed "s/\$/@example.com/"
}

for i in $(seq 1 $NUM_ADMINS_EXPECTED); do
    while true; do
        details=($(generate_details))
        name=${details[0]}
        email=${details[1]}

        result=$(psql -U "$POSTGRES_USER" -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -d "$POSTGRES_DB" -c "
        INSERT INTO users (name, email, password_hash, role)
        VALUES ('$name', '$email', '${PASSWORD_HASH}', 'Admin')
        ON CONFLICT (email) DO NOTHING;" 2>&1)

        if [[ $result == *"INSERT 0 1"* ]]; then
            # Insert successfully
            ((NUM_ADMINS_ACTUAL++))
            break
        else
            echo >&2 "Duplicate email found: $email. Regenerating details..."
            continue
        fi   
    done
done

for i in $(seq 1 $NUM_REVIEWERS_EXPECTED); do
    while true; do
        details=($(generate_details))
        name=${details[0]}
        email=${details[1]}

        result=$(psql -U "$POSTGRES_USER" -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -d "$POSTGRES_DB" -c "
            INSERT INTO users (name, email, password_hash, role)
            VALUES ('$name', '$email', '${PASSWORD_HASH}', 'Reviewer')
            ON CONFLICT (email) DO NOTHING;" 2>&1)

        if [[ $result == *"INSERT 0 1"* ]]; then
            # Insert successfully
            ((NUM_REVIEWERS_ACTUAL++))
            break
        else
            echo >&2 "Duplicate email found: $email. Regenerating details..."
            continue
        fi   
    done
done

echo "Seeded $NUM_ADMINS_ACTUAL Admins"
echo "Seeded $NUM_REVIEWERS_ACTUAL Reviewers"
