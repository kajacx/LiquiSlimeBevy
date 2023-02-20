#!/usr/bin/sh

./scripts/build-api.sh && \
./scripts/build-plugins.sh && \
./scripts/prepare-host.sh && \
\
echo "Running bevy game in web browser" && \
cd liquislime-bevy && \
cargo run --features bevy-host --target=wasm32-unknown-unknown && \
cd .. && \
\
echo "All done"
