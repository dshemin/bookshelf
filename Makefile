include .env

run:
	cargo watch -x 'run --bin bookshelf'

migrate:
	cargo run --bin migration -- -u ${BS_API_PG_CONN_URI}

generate: generate/entities

generate/entities:
	sea-orm-cli generate entity -u ${BS_API_PG_CONN_URI} -o entity/src --lib
