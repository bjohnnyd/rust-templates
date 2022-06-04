.DELETE_ON_ERROR:
.ONESHELL:
SHELL=bash
.SHELLFLAGS := -e -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

RUST_VERSION ?= 1.56.1

.PHONY: help all get__msrv

help: # Generates a list of all targets with their descriptions
	@grep ":" ./Makefile | awk "/#/ && !/grep/" | sed -E 's/(.+:).*#(.+)/\x1b[36m\1\x1b[0m\2/g'

all: get__msrv fmt test clippy check # format, test and lint

check: src/**rs # check if it builds
	cargo check

test: src/**rs # run all tests
	cargo test -q --all

fmt: src/**rs # Run all rustfmt
	cargo fmt

clippy: src/**rs
	cargo clippy --all-targets \
        --all-features -q \
        -- -D warnings

rust-toolchain.toml get__msrv: # Creates a toolchain file with the MSRV checked, test different version with env var $RUST_VERSION (SLOW)
	cargo install cargo-msrv
	cargo msrv --min $(RUST_VERSION) --toolchain-file
	mv rust-toolchain rust-toolchain.toml
	echo 'components = [ "rustfmt", "clippy" ]' >> rust-toolchain.toml

tree: # Shows the global feature dependency tree
	cargo install cargo-tree
	cargo tree

clean: # Cleans all intermediate or final entities
	@rm -r target || true
