#!/usr/bin/sh
set -e

# Run from root folder
rm -rf compile/compile-rust/user-plugin
cp -r liquislime-scripts/rust/rust-template compile/compile-rust/user-plugin

cd compile/compile-rust

docker build -t liquislime/compile-rust .
