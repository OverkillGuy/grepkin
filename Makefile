# Default make target, does everything useful, and installs pre-commit hooks
all: check lint install-precommit-hooks build run

check:
	cargo check

lint:
	cargo +nightly fmt
	cargo clippy

# lint:
# 	pre-commit run --all --all-files

build:
	cargo build

run:
	cargo run

test:
	cargo test

deb:
	cargo deb

# Less popular commands

install-precommit-hooks:
	pre-commit install

update-precommit:
	pre-commit autoupdate

build-docker:
	docker build -t grepkin .

PWD=$(shell pwd)
USERID=$(shell id -u)
GROUPID=$(shell id -g)

run-docker: build-docker
	docker run \
	    -w /app \
		-v "${PWD}/tests/:/app/tests/" \
		-v "${PWD}/features/:/app/features/" \
		-u "${USERID}:${GROUPID}" \
		grepkin

run-precommit-rust:
	pre-commit try-repo . grepkin --verbose --all-files

run-precommit-docker:
	pre-commit try-repo . grepkin-dockerfile --verbose --all-files

PHONY: all check lint install-precommit-hooks run build test deb update-precommit build-docker run-docker run-precommit-rust run-precommit-docker
