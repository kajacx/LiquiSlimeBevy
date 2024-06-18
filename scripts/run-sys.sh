#!/usr/bin/sh
set -e

# Run from parent folder

cd liquislime-scripts/as/as-template

# Uninstall package
head -27 package.json > p.json
echo '} }' >> p.json
mv p.json package.json
yarn install

#Install package
head -27 package.json > p.json
echo '"@wapc/as-msgpack": "/Programming/TypeScript/as-msgpack" } }' >> p.json
mv p.json package.json
yarn install

yarn asbuild
cd ../../..

cp ./liquislime-scripts/as/as-template/build/debug.wasm ./liquislime-bevy/assets/scripts/slime_clicker.wasm

echo "Running bevy game in debug mode"
cd liquislime-bevy && cargo run

echo "All done"
