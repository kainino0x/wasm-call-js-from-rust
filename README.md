# wasm-call-js-from-rust

Simple WebAssembly demo calling out from Rust to JavaScript

[**Live Demo**](https://kainino0x.github.io/wasm-call-js-from-rust/out/release/)
(You'll probably want to open dev tools.)

## Building

Requires Rust Nightly. Only tested on Linux.

```sh
make
# or
make debug
# or
make release
```

(default is `debug`)

## Running

```sh
cd out/debug  # (or out/release)
python2 -m SimpleHTTPServer
```
