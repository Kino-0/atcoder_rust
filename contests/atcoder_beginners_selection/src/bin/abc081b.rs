// ABC081B - Shift only

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let input = input.split_whitespace();

    let mut numbers: Vec<u32> = input.skip(1).map(|x| x.parse().unwrap()).collect();

    let mut count = 0;
    while numbers.iter().all(|x| x % 2 == 0) {
        count += 1;
        numbers.iter_mut().for_each(|x| *x /= 2);
    }

    println!("{count}");
}
