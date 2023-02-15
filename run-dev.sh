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
echo "Copying bindings to bevy host" && \
rm -rf                                                        liquislime-bevy/src/units/api_spec/types && \
cp -r liquislime-api/bindings/rust-wasmer-runtime/types       liquislime-bevy/src/units/api_spec/ && \
cp    liquislime-api/bindings/rust-wasmer-runtime/bindings.rs liquislime-bevy/src/units/api_spec/bindings.rs && \
echo "Bindings copied" && \
\
echo "Running bevy game in debug mode" && \
cd liquislime-bevy && \
cargo run --features bevy-host && \
cd .. && \
\
echo "All done"
