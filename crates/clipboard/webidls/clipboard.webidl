// https://w3c.github.io/clipboard-apis/#clipboard-interface

[SecureContext, Exposed=Window]
interface Clipboard : EventTarget {
  Promise<sequence<ClipboardItem>> read();
  Promise<DOMString> readText();
  Promise<void> write(sequence<ClipboardItem> data);
  Promise<void> writeText(DOMString data);
};

[
  // This constructor is different from the definition in the spec. It seems
  // that `wasm-bindgen` doesn't support Web IDL record types in constructors.
  // Use `any` as workaround.
  Constructor(any items),
  Exposed=Window
] interface ClipboardItem {
  // `wasm-bindgen` doen't seem to support `FrozenArray`, use sequence as
  // workaround.
  readonly attribute sequence<DOMString> types;
  Promise<Blob> getType(DOMString type);
};
