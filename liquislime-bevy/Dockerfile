FROM rust

# Warm up cargo
RUN cargo --version

# copy cargo stuff to compile external crates first
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .cargo .cargo
COPY rust-toolchain.toml rust-toolchain.toml

# Warm up cargo again?
RUN cargo --version

# compile external crates first, so they don't need to re-compile on every code change
RUN cargo build

# copy application code
COPY src src
COPY assets assets

# Run webpage
CMD cargo run --target wasm32-unknown-unknown
