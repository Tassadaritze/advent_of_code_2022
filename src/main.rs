mod solutions;

use crate::solutions::get_solution;
use std::{env, fs, process};

fn main() {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => {
            let out = get_solution(&arg).expect("Error getting solution");
            fs::write("output", out).expect("Error writing output");
        }
        None => {
            println!("Usage: cargo run [SOLUTION]");
            println!("Will run \"solutions/solution_[SOLUTION].rs\" on input data from \"input\".");
            process::exit(0);
        }
    }
}
