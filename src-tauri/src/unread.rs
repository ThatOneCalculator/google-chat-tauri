//! Unread-message badge for the tray icon.
//!
//! Google Chat reflects the unread count in the document title (e.g.
//! "(3) Google Chat"), which propagates to the OS window title. We poll that
//! title and, when the count changes, redraw the tray icon with a red badge
//! showing the number. This needs no JS injection into the remote page.

use ab_glyph::{point, Font, FontRef, PxScale, ScaleFont};
use image::{ImageFormat, RgbaImage};

/// Base tray icon (same asset referenced by tauri.conf.json's systemTray).
const BASE_ICON: &[u8] = include_bytes!("../icons/128x128.png");
/// Bundled font used to render the count (Noto Sans Bold, Apache-2.0;
/// see assets/badge-font.LICENSE).
const BADGE_FONT: &[u8] = include_bytes!("../assets/badge-font.ttf");

/// Extract an unread count from a window title. Returns the first
/// parenthesised integer (e.g. "(3) Google Chat" -> 3), or 0 if none.
pub fn parse_unread(title: &str) -> u32 {
    let mut rest = title;
    while let Some(open) = rest.find('(') {
        let after = &rest[open + 1..];
        if let Some(close) = after.find(')') {
            if let Ok(n) = after[..close].trim().parse::<u32>() {
                return n;
            }
            rest = &after[close + 1..];
        } else {
            break;
        }
    }
    0
}

/// Render the tray icon for the given unread `count`.
/// `count == 0` returns the plain base icon; otherwise a red badge with the
/// number (capped at "99+") is composited onto the top-right corner.
pub fn render_icon(count: u32) -> Option<tauri::Icon> {
    let mut img =
        image::load_from_memory_with_format(BASE_ICON, ImageFormat::Png)
            .ok()?
            .to_rgba8();

    if count > 0 {
        draw_badge(&mut img, count);
    }

    let (width, height) = img.dimensions();
    Some(tauri::Icon::Rgba {
        rgba: img.into_raw(),
        width,
        height,
    })
}

/// Composite the red badge + number onto the image in-place.
fn draw_badge(img: &mut RgbaImage, count: u32) {
    let (w, h) = img.dimensions();
    let r = (w as f32) * 0.32; // badge radius
    let cx = w as f32 - r; // hug the top-right corner
    let cy = r;

    // White halo ring for contrast against the app icon, then the red disc.
    fill_circle(img, cx, cy, r + 2.0, [255, 255, 255, 255]);
    fill_circle(img, cx, cy, r, [0xEA, 0x43, 0x35, 255]); // Google red

    let text = if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    };
    draw_text_centered(img, &text, cx, cy, r);
    let _ = h;
}

/// Anti-aliased filled circle, alpha-composited over the existing pixels.
fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
    let (w, h) = img.dimensions();
    let x0 = ((cx - radius - 1.0).floor().max(0.0)) as u32;
    let y0 = ((cy - radius - 1.0).floor().max(0.0)) as u32;
    let x1 = ((cx + radius + 1.0).ceil().min(w as f32)) as u32;
    let y1 = ((cy + radius + 1.0).ceil().min(h as f32)) as u32;

    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            // 1px anti-aliased edge.
            let cov = (radius + 0.5 - dist).clamp(0.0, 1.0);
            if cov > 0.0 {
                let a = (color[3] as f32 / 255.0) * cov;
                blend_pixel(img, x, y, [color[0], color[1], color[2]], a);
            }
        }
    }
}

/// Center `text` at (cx, cy), scaled to fit within a badge of radius `r`.
fn draw_text_centered(img: &mut RgbaImage, text: &str, cx: f32, cy: f32, r: f32) {
    let Ok(font) = FontRef::try_from_slice(BADGE_FONT) else {
        return;
    };

    // Scale down as the string gets longer so it stays inside the disc.
    let px = match text.chars().count() {
        1 => r * 1.5,
        2 => r * 1.15,
        _ => r * 0.85,
    };
    let scale = PxScale::from(px);
    let scaled = font.as_scaled(scale);

    // Lay glyphs left to right, accumulating advance.
    let mut pen_x = 0.0f32;
    let mut outlines = Vec::new();
    for ch in text.chars() {
        let id = font.glyph_id(ch);
        let glyph = id.with_scale_and_position(scale, point(pen_x, 0.0));
        pen_x += scaled.h_advance(id);
        if let Some(o) = font.outline_glyph(glyph) {
            outlines.push(o);
        }
    }
    if outlines.is_empty() {
        return;
    }

    // Union bounding box of all glyphs, to center the block precisely.
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    for o in &outlines {
        let b = o.px_bounds();
        min_x = min_x.min(b.min.x);
        min_y = min_y.min(b.min.y);
        max_x = max_x.max(b.max.x);
        max_y = max_y.max(b.max.y);
    }
    let off_x = cx - (min_x + (max_x - min_x) / 2.0);
    let off_y = cy - (min_y + (max_y - min_y) / 2.0);

    let (w, h) = img.dimensions();
    for o in &outlines {
        let b = o.px_bounds();
        o.draw(|gx, gy, cov| {
            if cov <= 0.0 {
                return;
            }
            let px = (b.min.x + gx as f32 + off_x).round();
            let py = (b.min.y + gy as f32 + off_y).round();
            if px >= 0.0 && py >= 0.0 && (px as u32) < w && (py as u32) < h {
                blend_pixel(img, px as u32, py as u32, [255, 255, 255], cov);
            }
        });
    }
}

/// Source-over composite of an opaque `color` at coverage `a` (0..1).
fn blend_pixel(img: &mut RgbaImage, x: u32, y: u32, color: [u8; 3], a: f32) {
    let p = img.get_pixel_mut(x, y);
    let bg_a = p[3] as f32 / 255.0;
    let out_a = a + bg_a * (1.0 - a);
    if out_a <= 0.0 {
        return;
    }
    for i in 0..3 {
        let src = color[i] as f32;
        let bg = p[i] as f32;
        p[i] = ((src * a + bg * bg_a * (1.0 - a)) / out_a).round() as u8;
    }
    p[3] = (out_a * 255.0).round() as u8;
}

#[cfg(test)]
mod tests {
    use super::parse_unread;

    #[test]
    fn parses_prefixed_count() {
        assert_eq!(parse_unread("(3) Google Chat"), 3);
        assert_eq!(parse_unread("Google Chat (12)"), 12);
        assert_eq!(parse_unread("Google Chat"), 0);
        assert_eq!(parse_unread("(no number) Google Chat (7)"), 7);
    }
}
