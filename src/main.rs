fn main() {
    // The text to render
    let text = include_str!("main.rs");
    let data = lib::make(800, text);
    lib::put_image("main", 800, data.len() as u32 / 800 / 3, &data);
}
