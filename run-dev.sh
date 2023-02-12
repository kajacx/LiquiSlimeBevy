#!/usr/bin/sh

echo "Building API protocol" && \
cd liquislime-api && \
cargo test && \
cargo run && \
rm bindings/rust-plugin/src/types.rs && \
cp -r src/types bindings/rust-plugin/src/types && \
rm bindings/rust-wasmer-runtime/types.rs && \
cp -r src/types bindings/rust-wasmer-runtime/types && \
cd bindings/rust-plugin && cargo check && cd .. && \
cd .. && \
echo "API protocol built" && \
\
echo "All done"
