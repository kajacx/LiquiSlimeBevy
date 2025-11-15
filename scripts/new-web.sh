#!/usr/bin/sh
set -e

# Run from parent folder

echo "Copying liquislime source"
rm -rf macroquad-webbuild/Cargo.toml
rm -rf macroquad-webbuild/crates
cp -r main-game/Cargo.toml macroquad-webbuild/
cp -r main-game/crates macroquad-webbuild/

echo "Compiling game for wasm32 target"
cd macroquad-webbuild
cargo build -p liquislime-macroquad --target wasm32-unknown-unknown

echo "Copying DEBUG wasm file"
# wasm-bindgen --out-dir ./liquislime-webserver/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm
cp ./target/wasm32-unknown-unknown/debug/liquislime-macroquad.wasm ../liquislime-docker/web-files/liquislime-macroquad.wasm
cd ..

echo "Copying assets"
rm -rf liquislime-docker/web-files/crates
mkdir -p liquislime-docker/web-files/crates/liquislime-macroquad
cp -r main-game/crates/liquislime-macroquad/assets liquislime-docker/web-files/crates/liquislime-macroquad

echo "Starting docker"
cd liquislime-docker
docker-compose up -d

echo "Done, view the game at http://127.0.0.1:8089/"
