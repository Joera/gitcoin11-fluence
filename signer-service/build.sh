#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd facade
cargo update --aggressive
marine build --release
cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp facade/target/wasm32-wasi/release/facade.wasm artifacts/signer_facade.wasm
scp artifacts/signer_facade.wasm root@gov:/opt/signer.wasm