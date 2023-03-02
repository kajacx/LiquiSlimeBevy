#!/usr/bin/sh

./scripts/build-api.sh && \
./scripts/build-plugins.sh && \
./scripts/prepare-host.sh && \
\
echo "Running bevy game in debug mode" && \
cd liquislime-bevy/src/units/api_spec && \
cat bindings.rs > bindings-original.rs && \
# cat bindings-sys.rs > bindings-fixed.rs && \
# tail -n +22 bindings.rs >> bindings-fixed.rs && \
# mv bindings-fixed.rs bindings.rs && \
cd ../../.. && \
cargo run --features bevy-host && \
cd .. && \
\
echo "All done"
