# globals
default: build
freshen: clean build
clean:
	cargo clean

# vars

# commands
build:
	cargo build

update:
	cargo update

ci:
	make-ci build $$(find src -name *.rs)

# tests
test: test-apriori
test-all:
	cargo test

test-apriori:
	cargo run --bin test_apriori

test-pcy:
	cargo run --bin test_pcy
