# Using Web APIs wasm-bindgen doesn't provide

This is an example of using Web APIs which aren't provided by `wasm-bindgen` but in a way similar to `wasm-bindgen`. Specifically this example uses a part of [async clipboard APIs](https://w3c.github.io/clipboard-apis/#async-clipboard-api). Note that this example only works on Chrome as other major browsers don't support async clipboard APIs.

## Usage

`wasm-pack` is required to run this example.

```sh
$ wasm-pack build --target web
$ python3 -m http.server
```

Then visit http://localhost:8000.

## How it works

The idea is to use a way similar to how `wasm-bindgen-webidl` generates bindings from Web IDL files. Steps:

- Create a dedicated crate for bindings. This is required to avoid using the same internal hash for generated bindings and code that uses the bindings.
- Put Web IDL definitions in the crate (`webidls/`).
- Generate bindings by using `wasm_bindgen_webidl::compile()` from a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
- Use the crate. See `src/lib.rs`.

## Caveats

This example is just for fun. I don't recommend you to do similar. If you want to add Web APIs, it would be better to follow the [instruction](https://rustwasm.github.io/wasm-bindgen/contributing/.web-sys/supporting-more-web-apis.html).
