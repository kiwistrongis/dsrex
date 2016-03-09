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

test-apriori:
	cargo build
	time -p target/debug/test_apriori data/subset0.dat

test-pcy:
	cargo build
	time -p target/debug/test_pcy data/subset0.dat

run-apriori:
	cargo build --release
	time -p target/release/apriori data/retail.dat

run-pcy:
	cargo build --release
	time -p target/release/pcy data/retail.dat
