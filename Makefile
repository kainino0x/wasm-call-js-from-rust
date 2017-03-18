PKG := wasm-call-js-from-rust
PKG_ := wasm_call_js_from_rust

# The garbage in this file is just to copy the build results into a
# deployable output directory (out/debug or out/release).

debug:
	cargo build
	mkdir -p out/debug
	cp html/index.html out/debug/index.html
	cp target/wasm32-unknown-emscripten/debug/deps/${PKG_}*.wasm out/debug/${PKG}.wasm
	sed 's/'${PKG_}'-[0-9a-f]\{16\}\./'${PKG}'\./' target/wasm32-unknown-emscripten/debug/${PKG}.js > out/debug/${PKG}.js

release:
	cargo build --release
	mkdir -p out/release
	cp html/index.html out/release/index.html
	cp target/wasm32-unknown-emscripten/release/deps/${PKG_}*.wasm out/release/${PKG}.wasm
	sed 's/'${PKG_}'-[0-9a-f]\{16\}\./'${PKG}'\./' target/wasm32-unknown-emscripten/release/${PKG}.js > out/release/${PKG}.js

clean:
	cargo clean
	rm -rf out

.PHONY: debug release clean
