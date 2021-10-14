#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd curl_adapter
cargo update --aggressive
marine build --release

cd ../local_storage
cargo update --aggressive
marine build --release

cd ../facade
cargo update --aggressive
marine build --release
cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp curl_adapter/target/wasm32-wasi/release/curl_adapter.wasm artifacts/
cp local_storage/target/wasm32-wasi/release/local_storage.wasm artifacts/
cp facade/target/wasm32-wasi/release/facade.wasm artifacts/
scp artifacts/facade.wasm root@gov:/opt/identity_service.wasm
#docker cp artifacts/facade.wasm fluence-ipfs:/opt/eth-account-service.wasm
#docker exec -ti fluence-ipfs ipfs add /opt/eth-account-service.wasm