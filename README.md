# wasm-call-js-from-rust

Simple WebAssembly demo calling out from Rust to JavaScript

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
