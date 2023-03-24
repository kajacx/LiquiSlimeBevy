#!/usr/bin/sh

./scripts/build-api.sh && \
./scripts/build-plugins-release.sh && \
./scripts/prepare-host.sh && \
\
echo "Running bevy game in release mode" && \
cd liquislime-bevy && \
cargo run --release && \
cd .. && \
\
echo "All done"
