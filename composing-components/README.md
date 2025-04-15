Usage:

```console
cd string-processor 
cargo component build --target wasm32-wasip2 --release                            
  Generating bindings for string-processor (src/bindings.rs)
    Finished `release` profile [optimized] target(s) in 0.01s
```

```console
cd ../orchestrator 
cargo clean
cargo component build --target wasm32-wasip2 --release
     Removed 72 files, 1.9MiB total
  Generating bindings for string-processor (src/bindings.rs)
   Compiling wit-bindgen-rt v0.41.0
   Compiling bitflags v2.9.0
   Compiling string-processor v0.1.0 (/Users/tpmccallum/composing-wasm-components/composing-components/string-processor)
    Finished `release` profile [optimized] target(s) in 1.05s
    Creating component target/wasm32-wasip1/release/string_processor.wasm
     Removed 0 files
error: failed to create a target world for package `orchestrator` (/Users/tpmccallum/composing-wasm-components/composing-components/orchestrator/Cargo.toml)

Caused by:
    0: failed to merge local target `/Users/tpmccallum/composing-wasm-components/composing-components/orchestrator/wit`
    1: package 'docs:string-processor' not found. known packages:
           docs:orchestrator
       
            --> /Users/tpmccallum/composing-wasm-components/composing-components/orchestrator/wit/world.wit:9:12
             |
           9 |     import docs:string-processor/processor;
             |            ^--------------------

```
