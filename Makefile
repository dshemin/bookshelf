RUSTFMT_FLAGS=--edition 2021
RUSTFMT_TARGETS=./**/*.rs

.PHONY: help
help:		## Show this help.
	@sed -ne '/@sed/!s/## //p' $(MAKEFILE_LIST)

.PHONY: backend
backend:	## Run backend in watch mode.
	cargo watch -x run

.PHONY: lint
lint: lint/rust	## Run all linters.

.PHONY: lint/rust
lint/rust:	## Run Ruse linters.
	rustfmt $(RUSTFMT_FLAGS) --check --files-with-diff $(RUSTFMT_TARGETS)
	cargo check
	cargo clippy

.PHONY: setup
setup:		## Prepeare just cloned repo for development.
	diesel --database-url "sqlite://./development/db" setup
