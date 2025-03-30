# Composing Components

## Build the components

### coordinate-normalizer

```bash
cd geolocation
echo "Building coordinate-normalizer..."
cd coordinate-normalizer
cargo component build --target wasm32-wasip1 --release
```

**Works:**

```bash
coordinate-normalizer % cargo component build --target wasm32-wasip1 --release
  Generating bindings for coordinate-normalizer (src/bindings.rs)
    Finished `release` profile [optimized] target(s) in 0.01s
```

### dms-converter

```bash
cd ../dms-converter
echo "Building dms-converter..."
cargo component build --target wasm32-wasip1 --release
```

**Error:**

```bash
dms-converter % cargo component build --target wasm32-wasip1 --release
error: failed to create a target world for package `dms-converter` (/Users/tpmccallum/composing-wasm-components/geolocation/dms-converter/Cargo.toml)

Caused by:
    0: failed to parse local target from directory `/Users/tpmccallum/composing-wasm-components/geolocation/dms-converter/wit`
    1: expected ')', found ':'
            --> /Users/tpmccallum/composing-wasm-components/geolocation/dms-converter/wit/world.wit:14:42
             |
          14 |     convert-to-dms: func(normalized: docs:normalizer/normalize/normalized-coordinates) -> dms-coordinates;
```

## Compose the components

```bash
cd ..
wac plug dms-converter/target/wasm32-wasip1/release/dms-converter.wasm --plug coordinate-normalizer/target/wasm32-wasip1/release/coordinate-normalizer.wasm -o final-app.wasm
```

```bash
cargo run --release
```
