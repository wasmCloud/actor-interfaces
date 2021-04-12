.PHONY: all deps build clean doc test

all: deps build

deps:

build:
	cargo build

# Rust builds accrue disk space over time (specifically the target directory),
# so running `make clean` should be done periodically.
clean:
	cargo clean
	rm -Rf build

doc:

test: build
	cargo test
