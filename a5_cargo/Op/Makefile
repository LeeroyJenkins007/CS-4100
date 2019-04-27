target/release/Op: src/main.rs
	cargo build --release

.PHONY: build

build: target/release/Op

.PHONY: run

run: target/release/Op
	./target/release/Op $F

.PHONY: clean

clean:
	cargo clean
