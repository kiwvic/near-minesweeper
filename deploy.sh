rm -r neardev

cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/minesweeper.wasm

source neardev/dev-account.env

near call $CONTRACT_NAME new --accountId $CONTRACT_NAME
