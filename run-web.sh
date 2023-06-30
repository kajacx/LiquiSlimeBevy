#!/usr/bin/sh

./scripts/build-plugins.sh && \
\
echo "Running bevy game in web browser" && \
cd liquislime-bevy && \
# cargo run --target=wasm32-unknown-unknown -- --initial-memory=268435456 --max-memory=268435456 && \
# cargo rustc --target=wasm32-unknown-unknown -- -Clink-arg=--initial-memory=268435456 -Clink-arg=--max-memory=268435456 && \
cargo build --target=wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./webserver/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm && \
# HACK to make jco work
cd assets/plugins && \
echo slime-spawner > slime-spawner-component.wasm && \
echo slime-voider > slime-voider-component.wasm && \
cd ../../.. && \
echo "Bevy game built in WASM" && \
\
echo "Restarting webserver" && \
cd liquislime-bevy/webserver && \
rm -rf assets && \
cp -r ../assets . && \
docker-compose down && docker-compose up -d && \
cd ../.. && \
\
echo "All done, play the game at http://127.0.0.1:8088/webserver/"
