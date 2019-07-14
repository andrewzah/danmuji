.PHONY: debug

debug:
	@cargo build

release:
	@cargo build --release

up:
	@cargo run

fmt:
	@cargo +nightly fmt

reset:
	@diesel database reset

seed: reset
	@psql -d danmuji_dev -f sql/seed.sql
