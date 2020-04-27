# build stage
FROM rust:latest as cargo-build

RUN apt-get update && apt-get install libpq-dev musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# final stage
FROM alpine:latest

RUN apk add postgresql-dev

RUN addgroup -g 1000 app
RUN adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /home/app/bin/

COPY --from=cargo-build /usr/src/app/target/x86_64-unknown-linux-musl/release/twitter-clone-rust .

RUN chown app:app twitter-clone-rust
USER app

EXPOSE 9090

CMD ["./twitter-clone-rust"]
