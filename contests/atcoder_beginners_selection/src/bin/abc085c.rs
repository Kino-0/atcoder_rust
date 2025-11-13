// ABC085C - Otoshidama

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("パース失敗"));
    let n = input.next().unwrap();
    let total_amount = input.next().unwrap();

    for z in 0..=n {
        for y in 0..=n {
            for x in 0..=n {
                if x + y + z == n && (10000 * x) + (5000 * y) + (1000 * z) == total_amount {
                    println!("{x} {y} {z}");
                    return;
                }
            }
        }
    }

    println!("-1 -1 -1");
}
