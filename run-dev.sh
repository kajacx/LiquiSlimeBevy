#!/usr/bin/sh

echo "Building API protocol" && \
cd liquislime-api && \
cargo test && \
cargo run && \
cd .. && \
echo "API protocol built" && \
\
echo "All done"
