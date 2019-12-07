// https://w3c.github.io/clipboard-apis/#clipboard-interface

[SecureContext, Exposed=Window]
interface Clipboard : EventTarget {
  Promise<DOMString> readText();
  Promise<void> writeText(DOMString data);
};
