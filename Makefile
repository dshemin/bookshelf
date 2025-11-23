RUSTFMT_FLAGS=--edition 2021
RUSTFMT_TARGETS=api/**/*.rs


.PHONY: help
help:		## Show this help
	@sed -ne '/@sed/!s/## //p' $(MAKEFILE_LIST)

.PHONY: lint
lint: lint/api

.PHONY: lint/api
lint/api:
	rustfmt $(RUSTFMT_FLAGS) --check --files-with-diff $(RUSTFMT_TARGETS)
	cargo check
	cargo clippy

.PHONY: cleanup
cleanup: 	## Remove all generated development files.
	rm -rf ./development/kanidm/data/*.pem
	rm -rf ./development/kanidm/data/kanidm*
	rm ./development/kanidm/data/*.pass

