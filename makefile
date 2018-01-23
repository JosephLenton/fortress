
cargo := cargo.exe
.DEFAULT_GOAL := default
default: make-dev



# 
# Things we can build and run.
#

run-fortress: make-dev
	./target/debug/fortress.exe \
			--map "./main/maps/map.map" 

run-generate: make-dev
	./target/debug/generate.exe \
			colour



# 
# Commit hooks.
#
# These are commands to run for git.
#
pre-commit: format-all test-all



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

make-game:
	make --no-print-directory make-one-dev manifest-path="./game/Cargo.toml"

make-generate:
	make --no-print-directory make-one-dev manifest-path="./generate/Cargo.toml"

make-head:
	make --no-print-directory make-one-dev manifest-path="./head/Cargo.toml"

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
make-one-dev: lint-one compile-one-dev test-one



# 
# Compile.
#
# This *only* builds the artiface. No tests. No lints. 
# Just build.
#
compile-one-release:
	$(cargo) build \
			--manifest-path="${manifest-path}" \
			--release

compile-one-dev:
	RUST_BACKTRACE="1"
	export RUST_BACKTRACE
	$(cargo) build \
			--manifest-path="${manifest-path}"



# 
# Linting.
#
# Uses Clippy.
#

lint: lint-all
lint-all:
	$(cargo) clippy \
		--all

lint-one:
	$(cargo) clippy \
			--manifest-path="${manifest-path}"



# 
# Run tests.
#

test: test-all
test-all:
	$(cargo) test \
		--all

test-one:
	RUST_BACKTRACE="1"
	export RUST_BACKTRACE
	$(cargo) test \
			--manifest-path="${manifest-path}"



# 
# Clean.
#

clean: clean-all
clean-all:
	$(cargo) clean



fmt: format
format-all:
	$(cargo) fmt \
		--all

format-one:
	$(cargo) fmt \
			--manifest-path="${manifest-path}"



