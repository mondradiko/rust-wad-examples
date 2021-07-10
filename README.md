# rust-wad-examples

## Getting Started

### Rust WebAssembly Target

Ensure that a WebAssembly target for Rust is installed. `wasm32-wasi` is useful
for debugging because it can log to the console, while `wasm32-unknown-unknown`
works for making minimal, standalone WebAssembly modules. Either can be used in
the following commands.

```bash
$ rustup target install wasm32-wasi
```

### Building

To build:
```bash
$ cargo build --target wasm32-wasi --release
```

### Testing

Wasmtime is used as the WebAssembly runtime to test it here, but Wasmer and WAVM
can also be used with very similar syntax.

To test:
```bash
$ wasmtime run --invoke wad_main target/wasm32-wasi/release/rust_wad_examples.wasm
```

## License

Battle Damaged Sci-fi Helmet - PBR by
[theblueturtle_](https://sketchfab.com/theblueturtle_), published under a
Creative Commons Attribution-NonCommercial license

https://sketchfab.com/models/b81008d513954189a063ff901f7abfe4
