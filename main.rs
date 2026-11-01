use std::io::{self, Read};

fn sigmoid(_x: f64) -> f64 {
    // TODO: return 1 / (1 + e^-x), without overflowing for large |x|.
    0.0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for raw in input.split('\n') {
        let s = raw.trim();
        if s.is_empty() {
            continue;
        }
        let x: f64 = s.parse().unwrap();
        println!("{:.4}", sigmoid(x));
    }
}
