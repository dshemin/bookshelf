include .env

run:
	cargo watch -x 'run --bin api'

migrate:
	sqlx migrate run  --database-url ${BS_API_PG_CONN_URI}