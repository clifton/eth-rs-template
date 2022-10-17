help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: bindings
bindings: ## Generate rust bindings for project contracts
	rm -rf bindings/src/bindings
	forge clean --root ./contracts
	git submodule update --recursive --remote
	forge build --root ./contracts --force --names --skip test --skip script
	forge bind --module --bindings-path ./bindings/src/bindings --root ./contracts --skip-build

.PHONY: lint
lint: ## Lint project
	cargo +nightly fmt --all -- --check
	forge fmt --root contracts --check
	cargo +nightly clippy --all --all-features -- -D warnings

.PHONY: format
format: ## Format project
	cargo +nightly fmt --all
	forge fmt --root contracts

.PHONY: test
test: ## Run tests
	forge test --root contracts
	cargo test --all --all-features


