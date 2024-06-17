#!/usr/bin/sh
set -e

# Run from parent folder

cd liquislime-scripts/assemblyscript/assemblyscript-template
yarn install
yarn asbuild
cd ../../..

cp ./liquislime-scripts/assemblyscript/assemblyscript-template/build/debug.wasm ./liquislime-bevy/assets/scripts/slime_clicker.wasm

echo "Running bevy game in debug mode"
cd liquislime-bevy && cargo run

echo "All done"
