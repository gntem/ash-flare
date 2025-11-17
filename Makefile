.PHONY: help test build clean release-major release-minor release-patch publish

help:
	@echo "Available targets:"
	@echo "  test           - Run all tests"
	@echo "  build          - Build the project"
	@echo "  clean          - Clean build artifacts"
	@echo "  release-major  - Bump major version, tag, and publish (x.0.0)"
	@echo "  release-minor  - Bump minor version, tag, and publish (0.x.0)"
	@echo "  release-patch  - Bump patch version, tag, and publish (0.0.x)"
	@echo "  publish        - Publish to crates.io (without version bump)"

test:
	@echo "Running tests..."
	cargo test

build:
	@echo "Building project..."
	cargo build --release

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

release-major: test
	@echo "Creating major release..."
	@current_version=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	major=$$(echo $$current_version | cut -d. -f1); \
	new_version=$$(($$major + 1)).0.0; \
	echo "Bumping version from $$current_version to $$new_version"; \
	sed -i "s/^version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	git add Cargo.toml; \
	git commit -m "chore: bump version to $$new_version"; \
	git tag -a "v$$new_version" -m "Release v$$new_version"; \
	echo "Tagged as v$$new_version"; \
	cargo publish; \
	git push && git push --tags

release-minor: test
	@echo "Creating minor release..."
	@current_version=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	major=$$(echo $$current_version | cut -d. -f1); \
	minor=$$(echo $$current_version | cut -d. -f2); \
	new_version=$$major.$$(($$minor + 1)).0; \
	echo "Bumping version from $$current_version to $$new_version"; \
	sed -i "s/^version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	git add Cargo.toml; \
	git commit -m "chore: bump version to $$new_version"; \
	git tag -a "v$$new_version" -m "Release v$$new_version"; \
	echo "Tagged as v$$new_version"; \
	cargo publish; \
	git push && git push --tags

release-patch: test
	@echo "Creating patch release..."
	@current_version=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	major=$$(echo $$current_version | cut -d. -f1); \
	minor=$$(echo $$current_version | cut -d. -f2); \
	patch=$$(echo $$current_version | cut -d. -f3); \
	new_version=$$major.$$minor.$$(($$patch + 1)); \
	echo "Bumping version from $$current_version to $$new_version"; \
	sed -i "s/^version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	git add Cargo.toml; \
	git commit -m "chore: bump version to $$new_version"; \
	git tag -a "v$$new_version" -m "Release v$$new_version"; \
	echo "Tagged as v$$new_version"; \
	cargo publish; \
	git push && git push --tags

publish:
	@echo "Publishing to crates.io..."
	@current_version=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	echo "Publishing version $$current_version"; \
	cargo publish
