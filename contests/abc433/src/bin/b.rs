// B - Nearest Taller
// https://atcoder.jp/contests/abc433/tasks/abc433_b

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");
    let tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("パース失敗"));
    let a: Vec<usize> = tokens.collect();
    let n = a[0];

    for i in 1..=n {
        for j in (1..=i).rev() {
            if a[i] < a[j] {
                println!("{j}");
                break;
            } else if j == 1 {
                println!("-1");
            }
        }
    }
}
