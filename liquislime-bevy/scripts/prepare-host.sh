#!/usr/bin/sh

echo "Copying bindings to bevy host" && \
rm -rf                                                        liquislime-bevy/src/units/api_spec/types && \
cp -r liquislime-api/bindings/rust-wasmer-runtime/types       liquislime-bevy/src/units/api_spec/ && \
cp    liquislime-api/bindings/rust-wasmer-runtime/bindings.rs liquislime-bevy/src/units/api_spec/bindings.rs && \
echo "Bindings copied"
