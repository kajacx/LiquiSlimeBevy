#!/usr/bin/sh

./scripts/build-api.sh && \
./scripts/build-plugins.sh && \
./scripts/prepare-host.sh && \
\
echo "Running bevy game in web browser" && \
cd liquislime-bevy/src/units/api_spec && \
head -n -1 bindings-web-wasmer3.rs > bindings-fixed.rs && \
#tail -n +47 bindings.rs >> bindings-fixed.rs && \
tail -n +38 bindings.rs >> bindings-fixed.rs && \
mv bindings-fixed.rs bindings.rs && \
cd ../../.. && \
cargo run --features bevy-host --target=wasm32-unknown-unknown && \
cd .. && \
\
echo "All done"
