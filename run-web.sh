#!/usr/bin/sh
set -e

./scripts/build-plugins.sh

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
cargo build --target=wasm32-unknown-unknown
wasm-bindgen --out-dir ./webserver/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm

echo "Bevy game built in WASM"

echo "Restarting webserver"
cd liquislime-webserver
docker-compose down && docker-compose up -d

echo "All done, play the game at http://127.0.0.1:8088/"
