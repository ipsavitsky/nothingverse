#!/bin/sh

DB_PATH="db/nothing.sqlite"
DB_URL="sqlite://${DB_PATH}"

mkdir -p "$(dirname "${DB_PATH}")"

if [ ! -f ${DB_PATH} ]; then
  cargo sqlx database create --database-url "${DB_URL}"
  cargo sqlx migrate run --source migrations --database-url "${DB_URL}"
fi

cargo sqlx prepare --workspace --database-url "${DB_URL}"
