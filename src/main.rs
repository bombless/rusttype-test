fn main() {
    let mut data = Vec::new();
    for _ in 0..10 {
        data.push(42);
        data.push(0);
        data.push(0)
    }
    let text = format!("{data:?}");
    let data = lib::make(800, &text);
    lib::put_image("main", 800, data.len() as u32 / 800 / 3, &data);
}
