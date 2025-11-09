// ABC081A Placing Marbles

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = input.chars().filter(|&x| x == '1').count();
    println!("{output}");
}
