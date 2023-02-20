#!/usr/bin/sh

echo "Building plugins" && \
cd liquislime-plugins && \
for plugin in */; do
  cd $plugin && \
  cargo build --target wasm32-unknown-unknown && \
  cd ..
done && \
cd .. && \
echo "Plugins built"
