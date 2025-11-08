// ABC086A - Product

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let tokens = input.split_whitespace();
    let prod: u32 = tokens.map(|x| x.parse::<u32>().unwrap()).product();
    if prod % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
