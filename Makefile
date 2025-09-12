.PHONY: help
help: ## Show this help message
	@echo "Legends of Legend - Project Management"
	@echo "======================================="
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""

.PHONY: build
build: ## Build the project in release mode
	cargo build --release

.PHONY: dev
dev: ## Build the project in debug mode
	cargo build

.PHONY: run
run: ## Run the game in debug mode
	cargo run

.PHONY: run-release
run-release: ## Run the game in release mode
	cargo run --release

.PHONY: check
check: ## Check the project for errors without building
	cargo check

.PHONY: test
test: ## Run all tests
	cargo test

.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

.PHONY: bench
bench: ## Run benchmarks
	cargo bench

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: fmt
fmt: ## Format the code
	cargo fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting without modifying files
	cargo fmt -- --check

.PHONY: lint
lint: ## Run clippy linter
	cargo clippy -- -D warnings

.PHONY: lint-fix
lint-fix: ## Run clippy and apply fixes
	cargo clippy --fix --allow-dirty --allow-staged

.PHONY: audit
audit: ## Check for security vulnerabilities in dependencies
	cargo audit

.PHONY: deps
deps: ## Update dependencies
	cargo update

.PHONY: docs
docs: ## Generate and open documentation
	cargo doc --open

.PHONY: docs-build
docs-build: ## Generate documentation without opening
	cargo doc

# Version management
.PHONY: version
version: ## Display current version
	@grep "^version" Cargo.toml | head -1 | cut -d'"' -f2

.PHONY: version-major
version-major: ## Bump major version (X.0.0)
	@current=$$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2); \
	major=$$(echo $$current | cut -d. -f1); \
	new_major=$$((major + 1)); \
	new_version="$$new_major.0.0"; \
	sed -i '' "s/version = \"$$current\"/version = \"$$new_version\"/" Cargo.toml; \
	echo "Version bumped from $$current to $$new_version"

.PHONY: version-minor
version-minor: ## Bump minor version (0.X.0)
	@current=$$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2); \
	major=$$(echo $$current | cut -d. -f1); \
	minor=$$(echo $$current | cut -d. -f2); \
	new_minor=$$((minor + 1)); \
	new_version="$$major.$$new_minor.0"; \
	sed -i '' "s/version = \"$$current\"/version = \"$$new_version\"/" Cargo.toml; \
	echo "Version bumped from $$current to $$new_version"

.PHONY: version-patch
version-patch: ## Bump patch version (0.0.X)
	@current=$$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2); \
	major=$$(echo $$current | cut -d. -f1); \
	minor=$$(echo $$current | cut -d. -f2); \
	patch=$$(echo $$current | cut -d. -f3); \
	new_patch=$$((patch + 1)); \
	new_version="$$major.$$minor.$$new_patch"; \
	sed -i '' "s/version = \"$$current\"/version = \"$$new_version\"/" Cargo.toml; \
	echo "Version bumped from $$current to $$new_version"

# Git helpers
.PHONY: tag
tag: ## Create a git tag with current version
	@version=$$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2); \
	git tag -a "v$$version" -m "Release version $$version"; \
	echo "Created tag v$$version"

.PHONY: push-tag
push-tag: ## Push the latest tag to remote
	@version=$$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2); \
	git push origin "v$$version"; \
	echo "Pushed tag v$$version"

# Development workflow
.PHONY: pre-commit
pre-commit: fmt lint test ## Run all checks before committing
	@echo "All pre-commit checks passed!"

.PHONY: ci
ci: fmt-check lint test ## Run all CI checks
	@echo "All CI checks passed!"

.PHONY: watch
watch: ## Watch for changes and rebuild
	cargo watch -x check -x test -x run

.PHONY: install-dev-tools
install-dev-tools: ## Install development tools
	@echo "Installing development tools..."
	@cargo install cargo-watch 2>/dev/null || true
	@cargo install cargo-audit 2>/dev/null || true
	@cargo install cargo-edit 2>/dev/null || true
	@echo "Development tools installed!"

.PHONY: info
info: ## Show project information
	@echo "Project: Legends of Legend"
	@echo "Version: $$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Edition: $$(grep "^edition" Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Rust:    $$(rustc --version)"
	@echo "Cargo:   $$(cargo --version)"

# Default target
.DEFAULT_GOAL := help