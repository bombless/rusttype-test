use rusttype::{Font, Scale, point};

fn make(c: char) -> [[char; 64]; 32] {
    // Load the font
    let font_data = include_bytes!("../../WenQuanYiMicroHei.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale { x: 64.0, y: 32.0 };

    let v_metrics = font.v_metrics(scale);
    // println!("v_metrics {v_metrics:?}");

    let mut data = [[' '; 64]; 32];
    let cursor = point(0.0, v_metrics.ascent);
    let glyph = font.glyph(c);
    let scaled = glyph.scaled(scale);
    let glyph = scaled.positioned(cursor);
    if let Some(bounding_box) = glyph.pixel_bounding_box() {
        // Draw the glyph into the image per-pixel by using the draw closure
        glyph.draw(|x, y, v| {
            let x = x as i32 + bounding_box.min.x + 1;
            let y = y as i32 + bounding_box.min.y;
            if x >= 0 && y >= 0 && x < 64 && y < 32 {
                let x = x as usize;
                let y = y as usize;
                let offset = (v * 3.99).floor() as usize;
                let candidates = [' ', '+', '$', '@'];
                data[y][x] = candidates[offset];
            } else {
                println!("Out of bounds: ({}, {}) limit (64, 32)", x, y,);
            }
        });
    }
    data
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: make <target>");
        return;
    }
    let target = &args[1];
    for c in target.chars() {
        let data = make(c);
        for line in data {
            for item in line {
                print!("{item}");
            }
            println!();
        }
    }
}
