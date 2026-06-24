// Clipboard image paste fallback for WebKitGTK.
//
// WebKitGTK doesn't put clipboard images into the page's `paste` event, so
// Google Chat never sees a pasted screenshot. We intercept paste, and when no
// image arrived natively, pull it from the OS clipboard via the Rust
// `read_clipboard_image` command and re-dispatch a synthetic paste carrying the
// image File into the composer.
(function () {
  if (window.__gcImagePaste) return;
  window.__gcImagePaste = true;

  function invoke(cmd) {
    return window.__TAURI__ && window.__TAURI__.invoke
      ? window.__TAURI__.invoke(cmd)
      : Promise.reject("no tauri");
  }

  function clipboardHasImage(cd) {
    if (!cd) return false;
    var list = cd.files && cd.files.length ? cd.files : cd.items;
    if (!list) return false;
    for (var i = 0; i < list.length; i++) {
      var t = list[i].type;
      if (t && t.indexOf("image/") === 0) return true;
    }
    return false;
  }

  async function fetchClipboardImageFile() {
    var dataUrl;
    try {
      dataUrl = await invoke("read_clipboard_image");
    } catch (e) {
      return null; // no image on clipboard (or read failed)
    }
    if (!dataUrl || dataUrl.indexOf("data:image") !== 0) return null;
    var blob = await (await fetch(dataUrl)).blob();
    return new File([blob], "pasted-image.png", { type: blob.type || "image/png" });
  }

  function dispatchPaste(target, file) {
    var dt = new DataTransfer();
    dt.items.add(file);
    var ev = new ClipboardEvent("paste", {
      clipboardData: dt,
      bubbles: true,
      cancelable: true,
    });
    // Some WebKit builds ignore the clipboardData init option; force it.
    try {
      Object.defineProperty(ev, "clipboardData", { value: dt });
    } catch (e) {}
    target.dispatchEvent(ev);
  }

  function hasTextData(cd) {
    if (!cd) return false;
    if (cd.types) {
      for (var i = 0; i < cd.types.length; i++) {
        var t = cd.types[i];
        if (t && t.indexOf("text") === 0) return true; // text/plain, text/html, ...
      }
    }
    return !!(cd.getData && (cd.getData("text/plain") || cd.getData("text/html")));
  }

  document.addEventListener(
    "paste",
    function (e) {
      // Only step in for the WebKitGTK image-only case: native image delivery
      // failed AND there's no text. Let every text/html paste through untouched.
      if (clipboardHasImage(e.clipboardData)) return;
      if (hasTextData(e.clipboardData)) return;
      var target = document.activeElement || e.target || document.body;
      e.preventDefault();
      e.stopImmediatePropagation();
      fetchClipboardImageFile().then(function (file) {
        if (file) dispatchPaste(document.activeElement || target, file);
      });
    },
    true
  );
})();
