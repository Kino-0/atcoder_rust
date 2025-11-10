// ABC087B - Coins

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut input = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();
    let x = input.next().unwrap();

    let mut counter = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if i * 500 + j * 100 + k * 50 == x {
                    counter += 1
                };
            }
        }
    }

    println!("{counter}");
}
