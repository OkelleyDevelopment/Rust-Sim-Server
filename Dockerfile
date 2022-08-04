FROM rust:1.58 as build

RUN USER=root cargo new --bin rusty-server
WORKDIR /rusty-server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./public ./public

RUN rm ./target/release/deps/rusty_server*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /rusty-server/target/release/rusty-server .

EXPOSE 8000

CMD ["./rusty-server"]
