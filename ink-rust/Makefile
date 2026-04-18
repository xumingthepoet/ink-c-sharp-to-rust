TIMEOUT := $(shell command -v timeout 2>/dev/null || command -v gtimeout 2>/dev/null)

.PHONY: gate fmt-check check test require-timeout

gate: fmt-check check test

fmt-check:
	cargo fmt --all -- --check

check:
	cargo check --workspace

test: require-timeout
	$(TIMEOUT) 15s cargo test -p ink-compiler
	$(TIMEOUT) 5s cargo test -p ink-runtime
	$(TIMEOUT) 5s cargo test -p inklecate
	$(TIMEOUT) 60s cargo test -p ink-tests

require-timeout:
	@test -n "$(TIMEOUT)" || (echo "error: install timeout or gtimeout" >&2; exit 1)
