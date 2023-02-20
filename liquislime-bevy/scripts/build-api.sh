#!/usr/bin/sh

echo "Building API protocol" && \
cd liquislime-api && \
cargo test --features api-generation && \
cargo run --features api-generation && \
\
rm -rf          bindings/rust-plugin/src/types* && \
cp -r src/types bindings/rust-plugin/src/types && \
\
rm -rf          bindings/rust-wasmer-runtime/types* && \
cp -r src/types bindings/rust-wasmer-runtime/types && \
\
cd bindings/rust-plugin && cargo check && cd ../.. && \
cd .. && \
echo "API protocol built"
