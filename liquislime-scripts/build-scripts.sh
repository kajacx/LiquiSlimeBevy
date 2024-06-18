#!/usr/bin/sh
set -e

# Run from parent folder
echo "Building scripts"

# Remove old files
rm -f liquislime-bevy/assets/scripts/*.wasm

./liquislime-scripts/rust/build-rust-scripts.sh
./liquislime-scripts/as/build-as-scripts.sh
