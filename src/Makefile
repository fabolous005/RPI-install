GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

build:
	$(CARGO_BIN) build --release --manifest-path ./lib/Cargo.toml
	# Built rust library
	$(GCC_BIN) -o file-listener ./main.c -Isrc  -L. -l:rust-file-listener/target/release/librust_file_listener.so
	# Built c binary

run: build
	./file-listener
