#!/usr/bin/sh

./scripts/build-plugins.sh && \
\
echo "Running bevy game in debug mode" && \
cd liquislime-bevy && \
cargo run && \
cd .. && \
\
echo "All done"
