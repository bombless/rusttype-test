const LEVELS: &[(char, f32)] = &[
    ('@', 0.125), // [0.5, 1]
    ('$', 0.080), // [0.25, 0.5)
    ('+', 0.034), // [0.125, 0.25)
                  // 添加更多级别...
];

fn get_level(input: f32) -> char {
    if input <= 0.0 || input > 1.0 {
        return ' ';
    }

    // 计算级别索引：基于 2^(-n) 的区间
    let index = if input == 1.0 {
        0
    } else {
        (-input.log2()).ceil() as usize - 1
    };

    LEVELS.get(index).copied().unwrap_or((' ', 0.0)).0
}

fn main() {
    for level in [0.75, 0.51, 0.5, 0.2, 0.1, 0.0] {
        println!("{:<6?} {:?}", level, get_level(level));
    }
}
