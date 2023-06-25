#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
echo "compiling evm_rpc..."
cd evm_rpc
cargo update --aggressive
marine build --release

echo "compiling meta_contract..."
cd ../meta_contract
cargo update --aggressive
marine build --release

cd ..
mkdir -p artifacts
rm -f artifacts/*.wasm

cp target/wasm32-wasi/release/evm_rpc.wasm artifacts/
cp target/wasm32-wasi/release/meta_contract.wasm artifacts/

marine aqua artifacts/meta_contract.wasm -s meta_contract -i meta_contract > ../aqua/meta_contract.aqua

RUST_LOG="info" mrepl --quiet Config.toml