use std::iter::repeat_n;

use image::{DynamicImage, Rgba};
use rusttype::{Font, Scale, point};

fn main() {
    // Load the font
    let font_data = include_bytes!("../../WenQuanYiMicroHei.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(32.0);

    // The text to render
    let text = include_str!("main.rs");

    let v_metrics = font.v_metrics(scale);

    for g in font.layout("a\nb\r\t", scale, point(20.0, 20.0 + v_metrics.ascent)) {
        println!("{g:?}")
    }

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    let mut data = vec![0u8; 800 * 800 * 3];
    let mut origin = point(0, 0);
    // Loop through the glyphs in the text, positing each one on a line
    let mut already_skipped = false;
    let mut first = true;
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            let skip = if already_skipped {
                false
            } else {
                already_skipped = glyph.id().0 == 0;
                already_skipped
            };
            if first {
                println!("skip {skip}");
                first = false;
            }
            if bounding_box.max.x + origin.x >= 800 {
                origin.x -= 800;
                origin.y += 32;
            }
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x + origin.x;
                let y = y as i32 + bounding_box.min.y + origin.y;
                if x >= 0 && y >= 0 && x < 800 && y < 800 {
                    let x = x as usize;
                    let y = y as usize;
                    data[(y * 800 + x) * 3] = (v * 255.0) as u8;
                    data[(y * 800 + x) * 3 + 1] = (v * 255.0) as u8;
                    data[(y * 800 + x) * 3 + 2] = (v * 255.0) as u8;
                } else {
                    println!("Out of bounds: ({}, {})", x, y);
                }
            });
        }
    }

    put_image(800, 800, &data);
}

fn put_image(width: u32, height: u32, data: &[u8]) {
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
    image.save("image_example.png").unwrap();
    println!("Generated: image_example.png");
}
