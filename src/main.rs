
use std::env;

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    println!("raw_args: {:?}", raw_args);
}
