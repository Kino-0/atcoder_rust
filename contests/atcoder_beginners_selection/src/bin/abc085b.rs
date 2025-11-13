// ABC085B - Kagami Mochi

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("パース失敗"));

    let _n = input.next().unwrap();

    let mut rice_cakes: Vec<i32> = input.collect();
    rice_cakes.sort_unstable();
    rice_cakes.dedup();
    let ans = rice_cakes.iter().count();

    println!("{ans}");
}
