#!/usr/bin/sh
set -e

# Run from parent folder
echo "Building plugins"

# Remove old files
rm -f liquislime-bevy/assets/plugins/*.wasm

# Build plugins
cd liquislime-plugins
for plugin in */; do
  plugin=`echo $plugin | sed -E s#/##`

  cd $plugin
  cargo component build --release
  cd ..
done

cp target/wasm32-wasi/release/*.wasm ../liquislime-bevy/assets/plugins/

echo "Plugins built"
