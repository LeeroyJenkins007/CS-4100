target/release/pa3: src/main.rs
	cargo build --release

.PHONY: build

build: target/release/pa3

.PHONY: run

run: target/release/pa3
	./target/release/gc $F

.PHONY: clean

clean:
	cargo clean
