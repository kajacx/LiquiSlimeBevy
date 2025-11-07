FROM rust

RUN rustup target install wasm32-unknown-unknown

COPY src src
COPY Cargo.toml Cargo.toml
COPY Rocket.toml Rocket.toml
COPY user-plugin user-plugin

CMD cargo run
