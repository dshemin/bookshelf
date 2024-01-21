include .env.development

RUSTFMT_FLAGS=--edition 2021
RUSTFMT_TARGETS=api/**/*.rs application/**/*.rs

.PHONY: run
run:
	cargo watch -x 'run --bin api'

.PHONY: fmt
fmt:
	rustfmt $(RUSTFMT_FLAGS) --emit files $(RUSTFMT_TARGETS)

.PHONY: lint
lint: lint/rustfmt lint/clippy lint/check lint/sql

.PHONY: lint/rustfmt
lint/rustfmt:
	rustfmt $(RUSTFMT_FLAGS) --check --files-with-diff $(RUSTFMT_TARGETS)

.PHONY: lint/clippy
lint/clippy:
	cargo clippy

.PHONY: lint/check
lint/check:
	cargo check

.PHONY: lint/sql
lint/sql:
	sqlfluff lint --dialect postgres migrations

.PHONY:devdeps
devdeps:
	pip3 install -r requirements.txt
	pre-commit install

.PHONY: migrate
migrate:
	sqlx migrate run --database-url ${BS_API_PG_CONN_URI}

.PHONY: keycloak/sync-realm
keycloak/sync-realm:
	docker compose exec keycloak /opt/keycloak/bin/kc.sh export --realm test --file /tmp/realm.json
	docker compose cp keycloak:/tmp/realm.json development/test_realm.json
