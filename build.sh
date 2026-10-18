#!/bin/sh
# Builds the WebAssembly module into web/pkg. Needs the wasm32 target and a
# wasm-bindgen CLI matching the crate version:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version 0.2.100
set -e
cd "$(dirname "$0")"
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir web/pkg \
  target/wasm32-unknown-unknown/release/chordwheel.wasm
if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -Os web/pkg/chordwheel_bg.wasm -o web/pkg/chordwheel_bg.wasm
fi
# Give the service worker a new cache name whenever the build changes.
hash=$(cat web/pkg/* web/app.js web/audio.js web/index.html web/manual.html web/style.css web/manifest.webmanifest | sha256sum | cut -c1-10)
sed -i "s/^const VERSION = .*/const VERSION = 'cw-$hash';/" web/sw.js
echo "Built web/pkg ($hash)"
