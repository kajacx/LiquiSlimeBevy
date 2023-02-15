#!/usr/bin/sh

echo "Building API protocol" && \
cd liquislime-api && \
cargo test --features api-generation && \
cargo run --features api-generation && \
\
rm -r bindings/rust-plugin/src/types* && \
cp -r src/types bindings/rust-plugin/src/types && \
\
rm -r bindings/rust-wasmer-runtime/types* && \
cp -r src/types bindings/rust-wasmer-runtime/types && \
\
cd bindings/rust-plugin && cargo check && cd ../.. && \
cd .. && \
echo "API protocol built" && \
\
echo "Building plugins" && \
cd liquislime-plugins && \
for plugin in */; do
  cd $plugin && \
  cargo build --target wasm32-unknown-unknown && \
  cd ..
done && \
cd .. && \
echo "Plugins built" && \
\
echo "Running bevy game in debug mode" && \
cd liquislime-bevy && \
cargo run && \
cd .. && \
\
echo "All done"
