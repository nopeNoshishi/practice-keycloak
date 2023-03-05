FROM rust:1.53.0

RUN apt-get update -y && apt-get upgrade -y
RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo install cargo-watch

WORKDIR /app

RUN cargo build --release