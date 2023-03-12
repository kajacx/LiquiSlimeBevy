#!/usr/bin/sh

./scripts/build-api.sh && \
./scripts/build-plugins.sh && \
./scripts/prepare-host.sh && \
\
echo "Running bevy game in debug mode" && \
cd liquislime-bevy && \
cargo run && \
cd .. && \
\
echo "All done"
