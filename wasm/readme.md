# Graph rs wasm

To build:

```
wasm-pack build --target web
http-server -c-1 .
```

See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

For size otpimization

See https://rustwasm.github.io/book/reference/code-size.html#compiling-with-link-time-optimizations-lto

```
brew install binaryen
```