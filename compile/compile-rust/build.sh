#!/usr/bin/sh
set -e

# Run from root folder
rm -rf compile/compile-rust/user-plugin
cp -r liquislime-scripts/rust/rust-template compile/compile-rust/user-plugin

cd compile/compile-rust

docker build -t liquislime/compile-rust .

# test that it works
# docker container stop compile-rust-test || true
# docker container rm compile-rust-test || true

# docker volume create --name compile-rust-target
# docker run --name compile-rust-test -v rust-compile-target:/target -p 8001:8000 -d liquislime/compile-rust

docker compose down
docker compose up -d


