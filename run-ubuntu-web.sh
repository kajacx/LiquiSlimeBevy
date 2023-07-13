#!/usr/bin/sh

echo "TODO: refactor this script"
exit 1

sh ./scripts/build-api.sh && \
sh ./scripts/build-plugins.sh && \
sh ./scripts/prepare-host.sh && \
\
echo "Running bevy game in web browser" && \
cd liquislime-bevy && \
# cargo run --target=wasm32-unknown-unknown -- --initial-memory=268435456 --max-memory=268435456 && \
# cargo rustc --target=wasm32-unknown-unknown -- -Clink-arg=--initial-memory=268435456 -Clink-arg=--max-memory=268435456 && \
cargo build -Zbuild-std=std,panic_abort --target=wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./webserver/ --target web ./target/wasm32-unknown-unknown/debug/liquislime-bevy.wasm && \
cd .. && \
echo "Bevy game built in WASM" && \
\
echo "Restarting webserver" && \
cd liquislime-bevy/webserver && \
rm -rf assets && \
cp -r ../assets . && \
sudo docker-compose down && sudo docker-compose up -d && \
cd ../.. && \
\
echo "All done, play the game at http://liquislime.com:8088/webserver/"
