include .env

run:
	cargo watch -x 'run --bin bookshelf'

migrate:
	cargo run --bin migration -- -u ${BS_API_PG_CONN_URI}
