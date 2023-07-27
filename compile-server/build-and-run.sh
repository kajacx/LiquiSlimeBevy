#!/usr/bin/sh
set -e

# Run from parent folder

rm -rf compile-server/liquislime-rust-api
cp -r liquislime-rust-api compile-server/

cd compile-server
sed -i -E 's#"../protocol.wit"#"../../protocol.wit"#' liquislime-rust-api/src/lib.rs

