use js_sys::{Object, Reflect};

// `build.rs` automatically generates bindings from Web IDL files and put
// outputs in a file which can be specified via the `BINDINGS` environment
// variable.
include!(env!("BINDINGS"));

/// A trait to add the `navigator.clipboard` attribute. This is effectively
/// the same as a partial interface defined in
/// https://w3c.github.io/clipboard-apis/#clipboard-interface
/// We can't use partial interfaces outside `wasm-bindgen` because of the
/// way how `wasm-bindgen` generates bindings.
pub trait NavigatorClipboard {
    fn clipboard(&self) -> Option<Clipboard>;
}

impl NavigatorClipboard for web_sys::Navigator {
    fn clipboard(&self) -> Option<Clipboard> {
        match Reflect::get(self.as_ref(), &"clipboard".into()) {
            Ok(clipboard) => Some(clipboard.into()),
            Err(_) => None,
        }
    }
}
