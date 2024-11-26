#!/usr/bin/env bash

# set -x # Enable debugging
set -eo pipefail 
# ^ Ensures the script exits immediately if any command fails (-e) and that the exit status of a pipeline 
# is determined by the last non-zero status

TOML_FILE="server/config/production.toml"

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

export POSTGRES_USER=$(get_toml_value "user")
export POSTGRES_PASSWORD=$(get_toml_value "password")
export POSTGRES_DB=$(get_toml_value "database")

docker-compose up -d --build
