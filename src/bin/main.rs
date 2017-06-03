extern crate base_emoji;

use std::env;

fn main() {
    let args = env::args().skip(1);

    for arg in args {
        print!("{} ", base_emoji::to_string(&arg));
    }
    println!();
}
