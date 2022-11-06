FROM rust:latest

RUN mkdir /app
WORKDIR /app

RUN cargo install cargo-watch wasm-pack

CMD ["cargo", "watch", "-x", "run"]