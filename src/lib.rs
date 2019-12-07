use wasm_bindgen::prelude::*;

use clipboard::NavigatorClipboard;

// Simple example: Just use generated API.

/// Reads text from clipboard asynchronously.
#[wasm_bindgen]
pub fn read_clipboard() -> JsValue {
    let window = web_sys::window().expect("window");
    let navigator = window.navigator();
    let clipboard = navigator.clipboard().expect("clipboard");
    let text = clipboard.read_text();
    text.into()
}

// A slightly more interesting example:

/// Creates a markdown style link, e.g., [title](http://example.com)
/// from text in the clipboard.
#[wasm_bindgen]
pub async fn markdown_link_from_clipboard() -> JsValue {
    // TODO: Implement.
    unimplemented!()
}
