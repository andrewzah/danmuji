.PHONY: debug

debug:
	@cargo build

release:
	@cargo build --release

up:
	@cargo run

docker:
	docker build -f docker/Dockerfile . -t andrewzah/danmuji

docker-clean:
	docker build -f docker/Dockerfile-alpine-rust docker -t andrewzah/alpine-rust --no-cache
	docker build -f docker/Dockerfile . -t andrewzah/danmuji --no-cache

docker-push:
	docker push andrewzah/alpine-rust
	docker push andrewzah/danmuji

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
