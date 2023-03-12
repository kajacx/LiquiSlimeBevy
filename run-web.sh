#!/usr/bin/sh

./scripts/build-api.sh && \
./scripts/build-plugins.sh && \
./scripts/prepare-host.sh && \
\
echo "Running bevy game in web browser" && \
cd liquislime-bevy && \
#cargo run --target=wasm32-unknown-unknown -- --initial-memory=268435456 --max-memory=268435456 && \
#cargo rustc --target=wasm32-unknown-unknown -- -Clink-arg=--initial-memory=268435456 -Clink-arg=--max-memory=268435456 && \
cargo build --target=wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm && \
cd .. && \
echo "Bevy game build in WASM" && \
\
echo "Restarting webserver" && \
cd liquislime-bevy/webserver && \
docker-compose down && docker-compose up -d && \
cd ../.. && \
\
echo "All done"
