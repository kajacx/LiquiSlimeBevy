#!/usr/bin/sh

echo "Building plugins" && \
rm -f liquislime-bevy/assets/plugins/*.wasm && \
rm -rf liquislime-bevy/assets/plugins/*-jco && \
cd liquislime-plugins && \
for plugin in */; do
  plugin=`echo $plugin | sed -E s#/##` && \
  cd $plugin && \
  cargo build --target wasm32-unknown-unknown && \
  cd target/wasm32-unknown-unknown/debug && mkdir -p out && \
  wasm-tools component new *.wasm -o out/$plugin-component.wasm && \
  jco transpile out/$plugin-component.wasm --instantiation -o out-jco && \
  cd ../../.. && \
  cp target/wasm32-unknown-unknown/debug/out/$plugin-component.wasm ../../liquislime-bevy/assets/plugins/ && \
  cp -r target/wasm32-unknown-unknown/debug/out-jco ../../liquislime-bevy/assets/plugins/$plugin-jco && \
  cd ..
done && \
cd .. && \
echo "Plugins built"
