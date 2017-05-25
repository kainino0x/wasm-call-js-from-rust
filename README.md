# Call JS from Rust in WebAssembly

Ever wondered how to do this super simple thing, which should be really easy
but for which there is almost no documentation? Wonder no more! In just a few
lines of arcane incantations in various languages, it's super easy!

(I kid! Only about 3 of the incantations are arcane!)

[**Live Demo**](https://kainino0x.github.io/wasm-call-js-from-rust/out/release/)
(Open dev tools to see the console logs.)

### Building

Requires Rust and Emscripten (obviously). Only tested on Linux.

```sh
make
# or
make debug
# or
make release
```

(Default is `debug`.)

### Running

```sh
cd out/debug  # (or out/release)
python2 -m SimpleHTTPServer
```

## Tutorial (Or: How To Do The Thing)

Really, there's so little code. Just take a look! (It's got comments, even.)
