#!/usr/bin/sh
set -e

# Run from parent folder
mode="$1"

echo "Copying liquislime source"
rm -rf liquislime-web/.cargo
rm -rf liquislime-web/src
rm -rf liquislime-web/Cargo.toml
cp -r liquislime-bevy/.cargo liquislime-web/
cp -r liquislime-bevy/src liquislime-web/
cp -r liquislime-bevy/Cargo.toml liquislime-web/
cp -r liquislime-bevy/rust-toolchain.toml liquislime-web/rust-toolchain.toml

echo "Copying liquislime assets"
rm -rf liquislime-web/liquislime-webserver/assets
cp -r liquislime-bevy/assets liquislime-web/liquislime-webserver/

echo "Running bevy game in web browser"
cd liquislime-web

cargo build $mode --target=wasm32-unknown-unknown
if [ "$mode" = "--release" ]; then
  echo "Copying RELEASE wasm file"
  wasm-bindgen --out-dir ./liquislime-webserver/ --target web ./target/wasm32-unknown-unknown/release/liquislime-bevy.wasm
else
  echo "Copying DEBUG wasm file"
  wasm-bindgen --out-dir ./liquislime-webserver/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm
fi

echo "Bevy game built in WASM"

