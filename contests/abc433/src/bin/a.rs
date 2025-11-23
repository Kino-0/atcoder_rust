// A - Happy Birthday! 4
// https://atcoder.jp/contests/abc433/tasks/abc433_a

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("パース失敗"));
    let mut x = tokens.next().expect("X取得失敗");
    let mut y = tokens.next().expect("Y取得失敗");
    let z = tokens.next().expect("Z取得失敗");

    while y <= 100 {
        if x == y * z {
            println!("Yes");
            return;
        }
        x += 1;
        y += 1;
    }
    println!("No");
}
