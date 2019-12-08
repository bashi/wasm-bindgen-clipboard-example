use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;

use clipboard::{ClipboardItem, NavigatorClipboard};

/// Reads text from clipboard asynchronously.
#[wasm_bindgen]
pub fn read_text_from_clipboard() -> JsValue {
    let window = web_sys::window().expect("window");
    let navigator = window.navigator();
    let clipboard = navigator.clipboard().expect("clipboard");
    let text = clipboard.read_text();
    text.into()
}

/// Fetches a content from the given url and copy the content to the clipboard.
#[wasm_bindgen]
pub async fn fetch_and_copy_to_clipboard(url: String) -> Result<(), JsValue> {
    let blob = fetch_as_blob(&url).await?;
    let navigator = web_sys::window().expect("window").navigator();
    let clipboard = navigator.clipboard().expect("clipboard");

    let items_obj = Object::new();
    Reflect::set(&items_obj, &blob.type_().into(), &blob)?;
    let item = ClipboardItem::new(&items_obj)?;
    let items = js_sys::Array::new();
    items.push(&item);
    JsFuture::from(clipboard.write(&items)).await?;

    Ok(())
}

async fn fetch_as_blob(url: &str) -> Result<Blob, JsValue> {
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().expect("window");
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response = response.dyn_into::<Response>().unwrap();

    let blob = JsFuture::from(response.blob()?).await?;
    let blob = blob.dyn_into::<Blob>().unwrap();

    Ok(blob)
}

/// Reads blobs from clipboard. This is basically the same as
/// https://web.dev/image-support-for-async-clipboard/#paste-image
#[wasm_bindgen]
pub async fn read_blobs_from_clipboard() -> Result<JsValue, JsValue> {
    let window = web_sys::window().expect("window");
    let navigator = window.navigator();
    let clipboard = navigator.clipboard().expect("clipboard");

    let items = JsFuture::from(clipboard.read()).await?;
    let items = items.dyn_into::<Array>()?;

    let blobs = Array::new();
    for item in items.values() {
        let item = item?.dyn_into::<ClipboardItem>().unwrap();
        let types = item.types().dyn_into::<Array>()?;
        for item_type in types.values() {
            let item_type = item_type?;
            // JsValue -> Rust's String.
            let type_string = item_type.as_string().unwrap();
            let blob = JsFuture::from(item.get_type(&type_string)).await?;
            blobs.push(&blob);
        }
    }

    Ok(blobs.into())
}
