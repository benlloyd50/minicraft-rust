cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rustcraft.wasm --out-dir rustcraft --no-modules --no-typescript
cp -r ./assets/ rustcraft/