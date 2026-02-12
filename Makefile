.PHONY: build test lint fmt sbom clean all bump-version

all: build sbom

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy

fmt:
	cargo fmt

sbom:
	@echo "Generating SPDX SBOM..."
	@mkdir -p target/sbom
	cargo sbom --output-format spdx_json_2_3 > target/sbom/sqlglot-rust.spdx.json
	@echo "SBOM written to target/sbom/sqlglot-rust.spdx.json"

clean:
	cargo clean

# Usage: make bump-version VERSION=1.2.3
bump-version:
ifndef VERSION
	$(error VERSION is required. Usage: make bump-version VERSION=1.2.3)
endif
	@echo "$(VERSION)" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$$' || \
		{ echo "Error: VERSION must be a full semantic version (MAJOR.MINOR.PATCH), e.g. 1.2.3"; exit 1; }
	@echo "Bumping version to $(VERSION)..."
	@# Update Cargo.toml
	sed -i '' 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml
	@# Update version in documentation
	sed -i '' 's/sqlglot-rust = "[^"]*"/sqlglot-rust = "$(VERSION)"/' README.md
	sed -i '' 's/sqlglot-rust = "[^"]*"/sqlglot-rust = "$(VERSION)"/' docs/installation.md
	@# Sync Cargo.lock
	cargo generate-lockfile
	@echo "Version updated to $(VERSION)"
