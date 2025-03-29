# Composing Components

## Build the components

```bash
echo "Building coordinate-normalizer..."
cd coordinate-normalizer
cargo component build --target wasm32-wasip1 --release
```

```bash
cd ../dms-converter
echo "Building dms-converter..."
cargo component build --target wasm32-wasip1 --release
```

## Compose the components

```bash
cd ..
wac plug dms-converter/target/wasm32-wasip1/release/dms-converter.wasm --plug coordinate-normalizer/target/wasm32-wasip1/release/coordinate-normalizer.wasm -o final-app.wasm
```

```bash
cargo run --release
```
