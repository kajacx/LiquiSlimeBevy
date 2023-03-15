#!/usr/bin/sh

# Run this script after you clone this repo for the first time
# to clone the forks of fp-bindgen and wasmer at correct branches.

echo "Cloning fp-bindgen"
git clone --branch wasmer3-kajacx https://github.com/kajacx/fp-bindgen.git
cd fp-bindgen && git remote add upstream https://github.com/fiberplane/fp-bindgen.git && git fetch --all && cd ..

echo "Cloning wasmer"
git clone --branch std-api-kajacx https://github.com/kajacx/wasmer.git
cd wasmer && git remote add upstream https://github.com/wasmerio/wasmer.git && git fetch --all && cd ..

read -n 1 -s -r -p "All done. Press any key to continue."
