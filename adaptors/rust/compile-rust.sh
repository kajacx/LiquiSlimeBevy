#!/usr/bin/sh
set -e

# Call from this directory

cd slime-clicker
cargo component build --target=wasm32-wasip2
cd ..