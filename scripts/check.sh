#!/usr/bin/sh

# Run from parent folder

./liquislime-scripts/build-scripts.sh

cd liquislime-scripts/rust
cargo clippy
cd ../..

cd liquislime-bevy
cargo check
cargo clippy
cargo test
cd ..
