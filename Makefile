.PHONY: debug

debug:
	@cargo build

release:
	@cargo build --release

fmt:
	@cargo +nightly fmt

reset:
	@diesel database reset
