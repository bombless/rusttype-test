fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: make <target>");
        return;
    }
    let target = &args[1];
    for c in target.chars() {
        println!("Target received {}", c);
    }
    lib::make(target);
}
