use ab_glyph::{ point, Font, FontRef, PxScale, ScaleFont };
use ksni::Icon;
use crate::config::config;

pub fn render_icon(text: &str, rgb: (u8, u8, u8)) -> Icon {
    // load font from assets
    let font = FontRef::try_from_slice(include_bytes!("../assets/DejaVuSans.ttf")).expect(
        "bundled font should be valid"
    );
    let cfg = config();

    let (w, h, font_px) = (cfg.icon_width, cfg.icon_height, cfg.font_px);

    let scale = PxScale::from(font_px);
    let scaled = font.as_scaled(scale); // gives ascent/descent/advance at this size
    let (r, g, b) = rgb;

    // straight-alpha RGBA buffer, fully transparent to start
    let mut rgba = vec![0u8; (w * h * 4) as usize];

    // center horizontally: sum the glyph advances, then offset
    let total: f32 = text
        .chars()
        .map(|c| scaled.h_advance(font.glyph_id(c)))
        .sum();
    let mut x = (((w as f32) - total) / 2.0).max(0.0);

    // baseline placed so the text block sits centered vertically
    let baseline = ((h as f32) + scaled.ascent() - scaled.descent().abs()) / 2.0;

    for ch in text.chars() {
        let gid = font.glyph_id(ch);
        let glyph = gid.with_scale_and_position(scale, point(x, baseline));
        if let Some(outline) = font.outline_glyph(glyph) {
            let bb = outline.px_bounds();
            outline.draw(|gx, gy, coverage| {
                let px = (bb.min.x as i32) + (gx as i32);
                let py = (bb.min.y as i32) + (gy as i32);
                if (0..w).contains(&px) && (0..h).contains(&py) {
                    let i = ((py * w + px) * 4) as usize;
                    let a = (coverage * 255.0) as u8;
                    if a > rgba[i + 3] {
                        rgba[i] = r;
                        rgba[i + 1] = g;
                        rgba[i + 2] = b;
                        rgba[i + 3] = a;
                    }
                }
            });
        }
        x += scaled.h_advance(gid);
    }

    // ksni wants ARGB32, network byte order, our buffer is RGBA, so rotate
    // each 4-byte pixel one step: [R,G,B,A] -> [A,R,G,B].
    for px in rgba.chunks_exact_mut(4) {
        px.rotate_right(1);
    }

    Icon { width: w, height: h, data: rgba }
}
