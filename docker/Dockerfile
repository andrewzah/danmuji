## builder
FROM andrewzah/alpine-rust as build

WORKDIR /build

RUN apk add --update postgresql-dev

RUN mkdir src \
 && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

COPY Cargo.toml .

RUN cargo build --release \
  && rm src/main.rs \
  && rm -rf target/release/deps

COPY ./src /build/src
COPY ./migrations /build/migrations
RUN cargo build --release \
  && rm -rf target/release/deps

## final
FROM alpine:latest
MAINTAINER Andrew Zah <zah@andrewzah.com>

RUN addgroup -g 1000 danmuji \
  && adduser -D -u 1000 -G danmuji danmuji_user

WORKDIR /app

COPY --from=build /build/target/release/danmuji .

RUN apk add --update libgcc postgresql-dev
COPY ./migrations /build/migrations

USER danmuji_user
CMD ["/app/danmuji"]
