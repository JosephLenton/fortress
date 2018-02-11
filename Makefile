
build_lint_allows :=
build_lint_warnings := -W unreachable-pub -W missing-docs
build_lint_denials := -D trivial-casts -D trivial-numeric-casts -D unused-extern-crates -D unused-import-braces

tools_lint_allows := 
tools_lint_warnings := -W unreachable-pub -W missing-docs
tools_lint_denials := -D trivial-casts -D trivial-numeric-casts -D unused-extern-crates -D unused-import-braces

BUILD_RUSTFLAGS := $(build_line_allows) $(build_lint_warnings) $(build_lint_denials)
TOOLS_RUSTFLAGS := $(tools_line_allows) $(tools_lint_warnings) $(tools_lint_denials)
RUST_BACKTRACE := 1

run_cmd := cmd.exe /C
run_cargo := cargo.exe
run_cargo_build := $(run_cmd) set RUST_BACKTRACE=$(RUST_BACKTRACE) \&\& set RUSTFLAGS=$(BUILD_RUSTFLAGS) \&\& $(run_cargo) +nightly
run_cargo_tools := $(run_cmd) set RUST_BACKTRACE=$(RUST_BACKTRACE) \&\& set RUSTFLAGS=$(TOOLS_RUSTFLAGS) \&\& $(run_cargo) +nightly

.DEFAULT_GOAL := default
default: make-dev



# 
# Things we can build and run.
#
run: run-fortress
run-fortress: make-main-no-test
	./target/debug/fortress.exe \
			--map "./main/maps/map.map" 

run-generate: make-main-no-test
	./target/debug/generate.exe \
			colour



# 
# Initialise.
#
init:
	cp ./pre-commit-hook.sh .git/hooks/pre-commit
	$(run_cargo_tools) install --force clippy
	rustup component add rustfmt-preview --toolchain=nightly



# 
# Commit hooks.
#
# These are commands to run for git.
#
pre-commit: format-all lint-all test-all



# 
# Make tasks.
#
# These do all the steps for making a dev or release target.
# i.e. lint, build, test.
#
make-release:
	make --no-print-directory make-one-release manifest-path="./main/Cargo.toml"

make-dev: make-main
make-main:
	make --no-print-directory make-one-dev manifest-path="./main/Cargo.toml"

make-main-no-test:
	make --no-print-directory make-one-dev-no-test manifest-path="./main/Cargo.toml"

make-game:
	make --no-print-directory make-one-dev manifest-path="./game/Cargo.toml"

make-generate:
	make --no-print-directory make-one-dev manifest-path="./generate/Cargo.toml"

make-head:
	make --no-print-directory make-one-dev manifest-path="./head/Cargo.toml"

make-llr:
	make --no-print-directory make-one-dev manifest-path="./llr/Cargo.toml"

make-util:
	make --no-print-directory make-one-dev manifest-path="./util/Cargo.toml"

make-world:
	make --no-print-directory make-one-dev manifest-path="./world/Cargo.toml"



# 
# Make One.
#
# These are like make, but don't have locations set.
# It's like a template of tasks to do.
#
# i.e. Lint, build, test, but it doesn't say which
# to lint, build, and test.
#
make-one-release: lint-one compile-one-release test-one
make-one-dev: make-one-dev-no-test test-one
make-one-dev-no-test: compile-one-dev



# 
# Compile.
#
# This *only* builds the artiface. No tests. No lints. 
# Just build.
#
compile-one-release:
	$(run_cargo_build) build \
			--manifest-path="${manifest-path}" \
			--release

compile-one-dev:
	$(run_cargo_build) build \
			--manifest-path="${manifest-path}"



# 
# Linting.
#
# Uses Clippy.
#

lint: lint-all
lint-all:
	$(run_cargo_tools) clippy \
			--all

lint-one:
	$(run_cargo_tools) clippy \
			--manifest-path="${manifest-path}"



# 
# Run tests.
#

test: test-all
test-all:
	$(run_cargo_build) test \
			--all

test-one:
	RUST_BACKTRACE="1"
	export RUST_BACKTRACE
	$(run_cargo_build) test \
			--manifest-path="${manifest-path}"



# 
# Clean.
#

clean: clean-all
clean-all:
	$(run_cargo_build) clean



# 
# Format.
#
# Formats all code and replaces it with new layout.
#

fmt: format
format: format-all
format-all:
	$(run_cargo_tools) fmt \
			--all

format-one:
	$(run_cargo_tools) fmt \
			--manifest-path="${manifest-path}"



