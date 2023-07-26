#!/usr/bin/sh
set -e

# Run from parent folder

echo "Running bevy game in debug mode"
cd liquislime-bevy && cargo run

echo "All done"
