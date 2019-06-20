use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    // NOTE: mut key words makes variable writable
    // Rust variables are immutable by default.
    let mut sum = 0;
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
        sum += i;
    }
    println!("The sum total: {}", sum);
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());

    say(out, width, &mut writer).unwrap();
}