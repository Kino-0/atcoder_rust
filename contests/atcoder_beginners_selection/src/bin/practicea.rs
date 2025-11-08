// PracticeA - Welcome to AtCoder

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_whitespace();
    let sumresult: i32 = tokens
        .by_ref()
        .take(3)
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    let text = tokens.next().unwrap();
    println!("{sumresult} {text}");
}
