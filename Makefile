help: ## Display this help screen
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

fmt: ## Format code (requires nightly)
	@rustup component add --toolchain nightly rustfmt 2>/dev/null || true
	@cargo +nightly fmt --all $(if $(CHECK),-- --check,)

clippy: ## Run clippy
	@cargo clippy --all-features -- -D warnings

cq: ## Run code quality checks (formatting + clippy)
	@$(MAKE) fmt CHECK=1
	@$(MAKE) clippy

check: ## Type-check
	@cargo check --all-features

test: ## Run tests
	@cargo test --release
	@cargo test --release -p dusk-bytes --no-default-features

no-std: ## Verify no_std compatibility on bare-metal target
	@rustup target add thumbv6m-none-eabi 2>/dev/null || true
	@cargo build --no-default-features -p dusk-bytes --target thumbv6m-none-eabi

doc: ## Generate docs
	@RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
	@RUSTDOCFLAGS="-D warnings" cargo doc -p dusk-bytes --no-default-features --no-deps

clean: ## Clean build artifacts
	@cargo clean

.PHONY: help fmt clippy cq check test no-std doc clean
