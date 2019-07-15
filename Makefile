.PHONY: debug

debug:
	@cargo build

release:
	@cargo build --release

up:
	@cargo run

fmt:
	@cargo +nightly fmt

test:
	@cargo test

bench:
	@rustup run nightly cargo bench

reset:
	@diesel database reset

seed: reset
	@psql -d danmuji_dev -f sql/seed.sql
