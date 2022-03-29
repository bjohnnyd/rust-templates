.DELETE_ON_ERROR:
.ONESHELL:
SHELL=bash
.SHELLFLAGS := -e -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: help rust__all check test fmt clippy npm__install start clean npm__build
help: # Generates a list of all targets with their descriptions
	@grep ":" ./Makefile | awk "/#/ && !/grep/" | sed -E 's/(.+:).*#(.+)/\x1b[36m\1\x1b[0m\2/g'

rust__all: fmt test clippy check # format, test and lint rust code

check: src/**rs # check if it rust code builds
	cargo check

test: src/**rs # run all rust tests
	cargo test -q --all

fmt: src/**rs # format rust code
	cargo fmt

clippy: src/**rs # lint rust code
	cargo clippy --all-targets \
        --all-features -q \
        -- -D warnings

build pkg/**: src/**rs # rebuilds the wasm file
	wasm-pack build

npm__init: pkg/** # initiales a wasm-app
	npm init -y wasm-app www

npm__install: www/**
	cd www && npm install

start: npm__install # serve npm server on http://127.1:8080
	cd www && npm run start

clean: # Cleans all intermediate or final entities
	@rm -r target || true
	@rm -rf www pkg || true

