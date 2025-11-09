fn main() {
    // The text to render
    let text = include_str!("main.rs");
    lib::make(text);
}
