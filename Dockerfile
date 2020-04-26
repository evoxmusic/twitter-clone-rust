FROM rust:latest as cargo-build

RUN apt-get update && apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

#COPY Cargo.toml Cargo.toml

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN addgroup -g 1000 myapp

RUN adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /home/app/bin/

COPY --from=cargo-build /usr/src/app/target/x86_64-unknown-linux-musl/release/app .

RUN chown app:app app

USER myapp

EXPOSE 9090

CMD ["./app"]
