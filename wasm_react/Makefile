.DELETE_ON_ERROR:
.ONESHELL:
SHELL=bash
.SHELLFLAGS := -e -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: help add_target rust__all check test fmt clippy npm__install start clean npm__build
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

npm__init node_modules/**: # needs to be only ran once to install everything
	npm i

add__wasm: # add wasm target to rustup
	rustup target add wasm32-unknown-unknown

target/wasm32-unknown-unknown/debug/{{crate_name}}.wasm: src/**rs
	cargo build --target wasm32-unknown-unknown

wrap_rust: target/wasm32-unknown-unknown/debug/{{crate_name}}.wasm # wrapts the rust generated target to prepare for react code
	wasm-bindgen target/wasm32-unknown-unknown/debug/{{crate_name}}.wasm --out-dir build

build/**: node_modules/** # builds the package
	npm run build
start: build/** # serve npm server on http://127.1:8080
	npm run dev

clean: # Cleans all intermediate or final entities
	@rm -r target || true
