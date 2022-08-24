cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/bevy_test_game.wasm --out-dir recent_web_build  --no-modules --no-typescript
