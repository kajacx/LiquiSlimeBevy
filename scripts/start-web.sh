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

echo "Copying liquislime assets"
rm -rf liquislime-web/liquislime-webserver/assets
cp -r liquislime-bevy/assets liquislime-web/liquislime-webserver/

echo "Running bevy game in web browser"
cd liquislime-web
# cargo run --target=wasm32-unknown-unknown -- --initial-memory=268435456 --max-memory=268435456
# cargo rustc --target=wasm32-unknown-unknown -- -Clink-arg=--initial-memory=268435456 -Clink-arg=--max-memory=268435456

cargo build "$mode" --target=wasm32-unknown-unknown
[ "$mode" = "--release" ]; if
  wasm-bindgen --out-dir ./liquislime-webserver/ --target web ./target/wasm32-unknown-unknown/release/liquislime-bevy.wasm
else
  wasm-bindgen --out-dir ./liquislime-webserver/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm
fi

echo "Bevy game built in WASM"

# echo "All done, play the game at http://127.0.0.1:8088/"
