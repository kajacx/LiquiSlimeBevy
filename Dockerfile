FROM rust

# copy cargo stuff to compile external crates first
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.toml
COPY .cargo .cargo
COPY rust-toolchain.toml rust-toolchain.toml

# compile external crates first, so they don't need to re-compile on every code change
RUN cargo build

# copy application code
COPY src src
COPY assets assets

# Run webpage
CMD cargo run --target wasm32-unknown-unknown
