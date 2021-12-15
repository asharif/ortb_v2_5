default:
	@cargo build --release

build-debug:
	@cargo fmt
	@cargo build

build-debug-linux:
	@CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --target=x86_64-unknown-linux-musl

unit-test:
	@cargo test
