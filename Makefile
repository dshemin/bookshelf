include .env

PATH_TO_MODELS="infrastructure/src/db/models"

run:
	cargo watch -x 'run --bin bookshelf'

migrate:
	cargo run --bin migration -- -u ${BS_API_PG_CONN_URI}

generate: generate/entities

generate/entities:
	sea-orm-cli generate entity -u ${BS_API_PG_CONN_URI} -o infrastructure/src/db/models
	rm \
		$(PATH_TO_MODELS)/mod.rs \
		$(PATH_TO_MODELS)/prelude.rs