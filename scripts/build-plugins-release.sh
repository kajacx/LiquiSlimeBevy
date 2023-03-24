#!/usr/bin/sh

echo "Building plugins" && \
rm liquislime-bevy/assets/plugins/*.wasm && \
cd liquislime-plugins && \
for plugin in */; do
  cd $plugin && \
  cargo build --release --target wasm32-unknown-unknown && \
  cp target/wasm32-unknown-unknown/release/*.wasm ../../liquislime-bevy/assets/plugins/
  cd ..
done && \
cd .. && \
echo "Plugins built"
