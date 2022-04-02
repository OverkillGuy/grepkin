# Default make target, does everything useful, and installs pre-commit hooks
all: check lint install-precommit-hooks build test deb

check:
	cargo check

lint:
	cargo +nightly fmt
	cargo clippy

# lint:
# 	pre-commit run --all --all-files

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

build-docker:
	docker build -t gherkin_testcomments .

PWD=$(shell pwd)
USERID=$(shell id -u)
GROUPID=$(shell id -g)

run-docker: build-docker
	docker run \
		-v "${PWD}/tests/:/tests/" \
		-v "${PWD}/features/:/features/" \
		-u "${USERID}:${GROUPID}" \
		gherkin_testcomments

.PHONY: all check lint install-precommit-hooks build test deb update-precommit build-docker run-docker
