//! Clipboard image bridge.
//!
//! WebKitGTK (wry's Linux backend) does not deliver clipboard images to a web
//! page's `paste` event, so pasting a screenshot into Google Chat silently does
//! nothing. The OS clipboard *does* hold the image — this command reads it
//! natively and hands it back as a data URL, which injected JS feeds into Chat's
//! composer (see `paste_inject.js`).
//!
//! On Wayland we shell out to `wl-paste`: `arboard` reads the X11/XWayland
//! clipboard (it prefers the X11 backend when `DISPLAY` is set), which is a
//! separate, often-stale clipboard from the real Wayland one. `arboard` is kept
//! as the fallback for X11, macOS and Windows.

use base64::Engine;
use std::io::Cursor;

/// Read an image from the system clipboard and return it as a
/// `data:image/...;base64,...` URL. Returns `Err` (with a short reason) when the
/// clipboard holds no image.
#[tauri::command]
pub fn read_clipboard_image() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    if std::env::var_os("WAYLAND_DISPLAY").is_some() {
        if let Ok(url) = read_via_wl_paste() {
            return Ok(url);
        }
    }
    read_via_arboard()
}

/// Read the Wayland clipboard via `wl-paste`, preferring PNG.
#[cfg(target_os = "linux")]
fn read_via_wl_paste() -> Result<String, String> {
    use std::process::Command;

    let mime = {
        let out = Command::new("wl-paste")
            .arg("--list-types")
            .output()
            .map_err(|e| e.to_string())?;
        let types = String::from_utf8_lossy(&out.stdout);
        if types.lines().any(|l| l.trim() == "image/png") {
            "image/png".to_string()
        } else {
            types
                .lines()
                .map(str::trim)
                .find(|l| l.starts_with("image/"))
                .ok_or_else(|| "no image on clipboard".to_string())?
                .to_string()
        }
    };

    let out = Command::new("wl-paste")
        .args(["--no-newline", "--type", &mime])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() || out.stdout.is_empty() {
        return Err("no image on clipboard".to_string());
    }

    let b64 = base64::engine::general_purpose::STANDARD.encode(&out.stdout);
    Ok(format!("data:{mime};base64,{b64}"))
}

/// Read the clipboard image via `arboard` and re-encode it as PNG.
fn read_via_arboard() -> Result<String, String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    let img = clipboard.get_image().map_err(|e| e.to_string())?;

    let buf = image::RgbaImage::from_raw(
        img.width as u32,
        img.height as u32,
        img.bytes.into_owned(),
    )
    .ok_or_else(|| "clipboard image buffer had unexpected size".to_string())?;

    let mut png = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(buf)
        .write_to(&mut png, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(png.into_inner());
    Ok(format!("data:image/png;base64,{b64}"))
}
