#!/usr/bin/sh
set -e

# Run from parent folder

cd adaptors
./compile-adaptors.sh
cd ..

cargo run \
    --manifest-path ../wasmi-component/Cargo.toml \
    -p wasmi-component-bindgen \
    -- \
    -m \
    main-game/crates/liquislime-wasmi/liquislime-api.wit \
    > main-game/crates/liquislime-wasmi/src/bindings.rs

cd main-game
cargo fmt -p liquislime-wasmi
cd ..
