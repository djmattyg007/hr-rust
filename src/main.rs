use std::env;

extern crate term_size;

fn main() {
    let mut width = 80;
    if let Some((w, _)) = term_size::dimensions() {
        width = w;
    }

    let mut lines: u64 = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        lines = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => 1,
        };
    }

    let mut outchar = String::with_capacity(width);
    for _ in 0..width {
        outchar += "#";
    }

    for _ in 0..lines {
        println!("{}", outchar);
    }
}
