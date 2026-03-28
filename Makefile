.PHONY: wasm clean

wasm:
	cd rsjson-wasm && wasm-pack build --target web
	cd rsjson-wasm && cp ./pkg/rsjson_wasm.js ../docs
	cd rsjson-wasm && cp ./pkg/rsjson_wasm_bg.wasm ../docs

clean:
	cd rsjson-wasm && rm -rf pkg