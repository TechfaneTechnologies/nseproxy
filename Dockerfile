FROM rust:1.62 as build

RUN USER=root cargo new --bin nseproxy
WORKDIR /nseproxy

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/nseproxy*
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libgssapi-krb5-2
COPY --from=build /nseproxy/target/release/nseproxy .

CMD ["./nseproxy"]
