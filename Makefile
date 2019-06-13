SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc
UPX := $(shell command -v upx 2> /dev/null)

.PHONY: all
all:
	cargo build --release 
	strip target/release/cbs
ifdef UPX
		upx target/release/cbs 
endif
	cargo install --path . 

.PHONY: build
build:
	cargo build --release 
	strip target/release/cbs
	upx target/release/cbs

.PHONY: build-linux
build-linux:
	cargo build --target x86_64-unknown-linux-musl --release
	strip target/x86_64-unknown-linux-musl/release/cbs
	upx target/x86_64-unknown-linux-musl/release/cbs

.PHONY: build-win
build-win:
	RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" cargo build --target x86_64-pc-windows-gnu --release
	strip target/x86_64-pc-windows-gnu/release/cbs.exe
	upx target/x86_64-pc-windows-gnu/release/cbs.exe

.PHONY: build-mac
build-mac:
	cargo build --target x86_64-apple-darwin --release
	strip target/x86_64-apple-darwin/release/cbs
	upx target/x86_64-apple-darwin/release/cbs

.PHONY: run
run:
ifndef ARGS
	@echo Run "make run" with ARGS set to pass arguments…
endif
	cargo run --release -- $(ARGS)

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean
