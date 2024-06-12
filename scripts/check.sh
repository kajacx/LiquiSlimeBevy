#!/usr/bin/sh

# Run from parent folder

./liquislime-scripts/build-scripts.sh

cd liquislime-scripts
cargo clippy
cd ..
