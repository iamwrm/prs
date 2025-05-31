.PHONY: help build test check docker_build docker_run docker_extract run clean update_deps

# Default target
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development targets
build: ## Build the project in release mode
	cargo build --release

test: ## Run tests
	cargo test

check: ## Run clippy and fmt checks
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

run: ## Run the application with default args
	cargo run -- -p top10-mem

update_deps: ## Check for dependency updates
	bash scripts/update_deps.sh

# Docker targets
docker_build: ## Build Docker image and extract binary
	bash scripts/docker_build.sh

docker_run: ## Run the Docker container
	docker run --rm -it prs prs -p top10-mem

docker_extract: ## Extract binary from Docker image
	docker run --rm -v ${PWD}:/host prs bash -c "cp /usr/local/bin/prs /host && chown 1000:1000 /host/prs"

# Utility targets
clean: ## Clean build artifacts
	cargo clean
	rm -f ./prs
	docker rmi -f rust_builder prs 2>/dev/null || true

