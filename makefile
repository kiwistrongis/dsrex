# globals
default: all
freshen: clean all
clean:
	rm -rf bin/* pkg/*
	cargo clean

# vars
pdf_files = \
	bin/report.pdf
pkg_file = \
	pkg/project1_kalev_100425828.tgz

# compilation definitions
$(pdf_files): bin/%.pdf : src/%.md src/metadata.yaml makefile
	pandoc \
		-r markdown+tex_math_single_backslash+link_attributes \
		--toc \
		--template=src/template.tex \
		src/metadata.yaml $< \
		-o $@

$(pkg_file): $(pdf_files)
	cp $(pdf_files) .
	tar -cvf $(pkg_file) \
		bin/ \
		data/retail.dat \
		data/subset_*.dat \
		src/ \
		target/debug \
		target/release \
		results/ \
		pkg/.gitignore \
		Cargo.lock \
		Cargo.toml \
		cargo.toml \
		makefile \
		readme.md \
		report.pdf
	rm -rf *.pdf

# commands
all: build build-release report
build:
	cargo build
build-release:
	cargo build --release
report: $(pdf_files)
package: build build-release $(pkg_file)

update:
	cargo update

ci-build:
	make-ci build $$(find src -name *.rs)
ci-report:
	make-ci bin/report.pdf src/report.md src/metadata.yaml

# tests
test: test-pcy

test-apriori: build
	time -p target/debug/test_apriori data/subset0.dat
test-pcy: build
	time -p target/debug/test_pcy data/subset0.dat

run-apriori: build-release
	time -p target/release/apriori data/retail.dat
run-pcy: build-release
	time -p target/release/pcy data/retail.dat

test-report: bin/report.pdf
	evince $<
