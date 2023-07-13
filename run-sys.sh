#!/usr/bin/sh
set -e

./scripts/build-plugins.sh

echo "Running bevy game in debug mode"
cd liquislime-bevy && cargo run

echo "All done"
