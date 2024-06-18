#!/bin/usr/sh
set -e

# Run from root folder
cd liquislime-scripts/as/as-template

# Uninstall package
head -27 package.json > p.json
echo '} }' >> p.json
mv p.json package.json
yarn install

#Install package
head -27 package.json > p.json
echo '    "@wapc/as-msgpack": "/Programming/TypeScript/as-msgpack"' >> p.json
echo '  }' >> p.json
echo '}' >> p.json
mv p.json package.json
yarn install

yarn asbuild
