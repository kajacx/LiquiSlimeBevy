#!/usr/bin/sh
set -e

# Run from parent folder
echo "Building plugins"

# Remove old files
rm -f liquislime-bevy/assets/plugins/*.wasm

# Prepare plugins
cd liquislime-plugins
for plugin in */; do
  plugin=`echo $plugin | sed -E s#/##`
  if [ "$plugin" = "target" ] || [ "$plugin" = "plugin-template" ]; then
    continue
  fi

  echo "Preparing plugin $plugin"

  crate_name=`grep 'name =' "$plugin/Cargo.toml" | sed -E 's/.*"(.*)".*/\1/'`
  cp plugin-template/Cargo.toml "$plugin/Cargo.toml"
  sed -E -i 's/name =.*/name = "'"$crate_name"'"/' "$plugin/Cargo.toml"
  sed -E -i 's/package =.*/package = "liquislime:'"$plugin"'"/' "$plugin/Cargo.toml"

  rm -rf "$plugin/src/settings" "$plugin/src/types"
  cp -r plugin-template/src/types "$plugin/src"
  cp -r plugin-template/src/settings "$plugin/src"
  cp plugin-template/src/lib.rs "$plugin/src/lib.rs"
  cp plugin-template/.gitignore_ "$plugin/.gitignore"
done

# Build plugins
for plugin in */; do
  plugin=`echo $plugin | sed -E s#/##`
  if [ "$plugin" = "target" ] || [ "$plugin" = "plugin-template" ]; then
    continue
  fi

  echo "Building plugin $plugin"
  cd $plugin
  cargo component build --target=wasm32-unknown-unknown --release
  cd ..
done

cp target/wasm32-unknown-unknown/release/*.wasm ../liquislime-bevy/assets/plugins/

echo "Plugins built"
