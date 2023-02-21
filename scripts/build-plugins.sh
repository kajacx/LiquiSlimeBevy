#!/usr/bin/sh

echo "Building plugins" && \
cd liquislime-plugins && \
for plugin in */; do
  cd $plugin && \
  cargo build --target wasm32-unknown-unknown && \
  cp target/wasm32-unknown-unknown/debug/*.wasm ../../liquislime-bevy/assets/plugins/
  cd ..
done && \
cd .. && \
echo "Plugins built"
