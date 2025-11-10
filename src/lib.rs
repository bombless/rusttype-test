use std::iter::repeat_n;

use image::{DynamicImage, Rgba};
use rusttype::{Font, Scale, point};

pub fn make(width: usize, text: &str) -> Vec<u8> {
    // Load the font
    let font_data = include_bytes!("../../WenQuanYiMicroHei.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(28.0);

    let v_metrics = font.v_metrics(scale);
    // println!("v_metrics {v_metrics:?}");

    let mut data = vec![0u8; width * 32 * 3];
    let mut line_count = 1;
    let mut cursor = point(0.0, v_metrics.ascent);
    let mut last_glyph = None;
    for c in text.chars() {
        let glyph = font.glyph(c);
        let id = glyph.id();
        let scaled = glyph.scaled(scale);

        let w = scaled.h_metrics().advance_width;
        let is_new_line = c == '\n' || cursor.x.ceil() + w >= width as f32;
        if is_new_line {
            cursor.x = 0.0;
            cursor.y += 32.0;
            line_count += 1;
            data.extend(repeat_n(0, width * 32 * 3))
        }
        if !is_new_line && let Some(last) = last_glyph {
            cursor.x += font.pair_kerning(scale, last, id);
        }
        let glyph = scaled.positioned(cursor);
        if !is_new_line {
            cursor.x += w;
        }
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            if c == '\n' {
                continue;
            }
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x;
                let y = y as i32 + bounding_box.min.y;
                if x >= 0 && y >= 0 && x < width as i32 && y < line_count * 32 {
                    let x = x as usize;
                    let y = y as usize;
                    data[(y * width + x) * 3] = (v * 255.0) as u8;
                    data[(y * width + x) * 3 + 1] = (v * 255.0) as u8;
                    data[(y * width + x) * 3 + 2] = (v * 255.0) as u8;
                } else {
                    println!(
                        "Out of bounds: ({}, {}) limit ({}, {})",
                        x,
                        y,
                        width,
                        line_count * 32
                    );
                }
            });
        }
        last_glyph = Some(id);
    }

    data
}

pub fn put_image(filename: &str, width: u32, height: u32, data: &[u8]) {
    let mut image = DynamicImage::new_rgba8(width, height).to_rgba8();
    for i in 0..width {
        for j in 0..height {
            let offset = j as usize * width as usize * 3 + i as usize * 3;
            image.put_pixel(
                i,
                j,
                Rgba([data[offset], data[offset + 1], data[offset + 2], 255]),
            )
        }
    }
    // Save the image to a png file
    image.save(&format!("target/{filename}.png")).unwrap();
    println!("Generated: target/{filename}.png");
}
