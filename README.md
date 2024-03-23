# Liquislime!

A video game inspired by the Creeper World video games written in Bevy.

## Customizable unit behaviour

A core feature of the game is (or will be) the ability to program unit behavior in Rust, and then
load and the resulting code at runtime using WASM.

To run the WASM modules both on desktop and on the web, I am using [`wasm-bridge`](https://github.com/kajacx/wasm-bridge),
which I created and maintain for this purpose.

## Project setup guide

If you want to clone this repo and run Liquislime for yourself, here is what you need to do:

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install the nightly toolchain with `rustup toolchain install nightly`
- Add `wasm32-unknown-unknown` compilation target with `rustup target add wasm32-unknown-unknown`
- Install [llvm](https://releases.llvm.org/download.html)
- On Ubuntu, install `opensll`, `pkg-config`, `clang` and `build-essentials`
- On Windows, install Microsoft Visual C++ Redistributable
- Install [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) (needed for web target only)
- Install [Docker](https://docs.docker.com/get-docker/), including `docker-compose`
  (optional for web target, you can host the generated files yourself)

Then:

- Clone this repository
- Run the `setup.sh` script to clone the two forked repositories
- Run the `run-dev.sh` script to verify that the desktop build works
- Run the `run-web.sh` script to verify that the web build works
