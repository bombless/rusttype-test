const FACTORS: [(char, f32); 10] = [
    ('@', 0.125),
    ('#', 0.087),
    ('$', 0.080),
    ('*', 0.045),
    ('+', 0.034),
    ('=', 0.036),
    ('o', 0.061),
    ('c', 0.044),
    ('0', 0.079),
    ('.', 0.007),
];

struct FactorFinder(Vec<(char, f32)>);
impl FactorFinder {
    fn new() -> Self {
        let mut ret = FACTORS.to_vec();
        ret.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        FactorFinder(ret)
    }
    fn for_factor(&self, factor: f32) -> char {
        if factor < 0.01 {
            return ' ';
        }
        let mut idx = 0;
        for i in 0..self.0.len() {
            if self.0[i].1 < factor {
                idx = i;
                break;
            }
        }
        self.0[idx].0
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: make <target>");
        return;
    }
    let target = &args[1];
    let factor_finder = FactorFinder::new();
    for c in target.chars() {
        println!("target {c}");
        let text = format!("{c}");
        let data = lib::make(32, &text);
        let count = 32.0 * 32.0;
        let mut avg = 0.0;

        for j in 0..32 {
            for i in 0..32 {
                let idx = j * 32 + i;
                let factor = data[idx * 3] as f32 / 256.0;
                avg += data[idx * 3] as f32 / 256.0 / count;
                let c = factor_finder.for_factor(factor);
                print!("{c}{c}");
            }
            println!();
        }

        println!("factor {avg:.3}");
        lib::put_image(&text, 32, data.len() as u32 / 32 / 3, &data);
    }
}
