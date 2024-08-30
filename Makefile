install: install-git-hooks

install-git-hooks:
	cp githooks/* .git/hooks/
	chmod +x .git/hooks/*

format:
	cargo fmt --all --
.PHONY: format

format-check:
	cargo fmt --all -- --check
.PHONY: format-check

test:
	cargo test --all --all-features --tests
.PHONY: test

test-all: test
.PHONY: test-all

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy

doc:
	cargo doc --all-features
.PHONY: doc

all: format clippy test doc

build:
	cargo build
.PHONY: build

build-release:
	cargo build --release
.PHONY: build-release