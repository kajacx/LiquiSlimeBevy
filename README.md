# Liquislime!

A video game inspired by the Creeper World video games written in Bevy.

## Customizable unit behaviour

A core feature of the game is (or will be) the ability to program unit behaviour in Rust, and then
load and the resulting code at runtime using WASM.

Currently I am using a [custom fork](https://github.com/kajacx/fp-bindgen) of [fp-bindgen](https://github.com/fiberplane/fp-bindgen),
with a [custom fork](https://github.com/kajacx/wasmer) of [wasmer](https://github.com/wasmerio/wasmer) to make it work.

## Problems

The web build runs out of memory after ~5 seconds, thowing an error in the console and freezing. The desktop build also runs out of memory after ~30 seconds.

There is also a horribly unsafe `impl Send` and `impl Sync` for a type that shouldn't have it, so run the code at your own risk.

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
