# DATABASE
.PHONY: db_clean db_migrate db_create db_prepare
DB_NAME = ah_bonus

db: db_create db_migrate

db_create: db_clean
	sqlx database create

db_migrate:
	sqlx migrate run

db_clean:
	rm -f ${DB_NAME}.db ${DB_NAME}.db-shm ${DB_NAME}.db-wal ${DB_NAME}_dev.db ${DB_NAME}_dev.db-shm ${DB_NAME}_dev.db-wal

db_prepare:
	cargo sqlx prepare --workspace -- --all-features --all-targets
# TELEGRAM BOT
include .env
ENV = RUST_LOG=debug TELOXIDE_TOKEN=${TELOXIDE_TOKEN}
RUN_BOT = ${ENV} cargo run -p telegram_bot
RUN_BOT_FLAGS = -d ${DATABASE_URL}
run:
	${RUN_BOT} --bin bot -- ${RUN_BOT_FLAGS}

notify:
	${RUN_BOT} --bin notify -- ${RUN_BOT_FLAGS}

build:
	cargo build
