#!/usr/bin/sh
set -e

# Run from root folder
echo "Building Rust scripts"

# Prepare scripts
cd liquislime-scripts/rust/scripts
for script in */; do
  script=`echo $script | sed -E s#/##`
  echo "Preparing script $script"

  crate_name=`grep 'name =' "$script/Cargo.toml" | sed -E 's/.*"(.*)".*/\1/'`
  cp ../rust-template/Cargo.toml "$script/Cargo.toml"
  sed -E -i 's/name =.*/name = "'"$crate_name"'"/' "$script/Cargo.toml"

  rm -rf "$script/src/types" "$script/src/settings" "$script/src/api"
  cp -r ../rust-template/src/types "$script/src"
  cp -r ../rust-template/src/settings "$script/src"
  cp -r ../rust-template/src/api "$script/src"
  cp ../rust-template/src/lib.rs "$script/src/lib.rs"
  cp ../rust-template/.gitignore_ "$script/.gitignore"
done

# Build scripts
for script in */; do
  script=`echo $script | sed -E s#/##`
  echo "Building script $script"

  cd $script
  cargo build --target=wasm32-unknown-unknown
  cd ..
done

cd ..

# Build template
echo "Building scritp template"

cd rust-template
cargo build --target=wasm32-unknown-unknown
cd ..

cp target/wasm32-unknown-unknown/debug/*.wasm ../../liquislime-bevy/assets/scripts/

echo "Rust scripts built"
