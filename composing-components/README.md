Usage:

```console
cd string-processor 
cargo component build --target wasm32-wasip2 --release                            
  Generating bindings for string-processor (src/bindings.rs)
    Finished `release` profile [optimized] target(s) in 0.01s
cd ../orchestrator 
cargo component build --target wasm32-wasip2 --release
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
