# Default make target, does everything useful, and installs pre-commit hooks
all: check lint install-precommit-hooks build test deb

check:
	cargo check

lint:
	pre-commit run --all --all-files

build:
	cargo build

test:
	cargo test

deb:
	cargo deb

# Less popular commands

install-precommit-hooks:
	pre-commit install

update-precommit:
	pre-commit autoupdate

.PHONY: all check lint install-precommit-hooks build test deb update-precommit
