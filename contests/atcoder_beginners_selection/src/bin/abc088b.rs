// ABC088B - Card Game for Two

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読み込み失敗");
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<u16>().expect("パース失敗"));
    let _n = input.next().unwrap();

    let mut cards: Vec<u16> = input.collect();
    cards.sort_unstable();
    cards.reverse();
    let alice_score: u16 = cards.iter().step_by(2).sum();
    let bob_score: u16 = cards.iter().skip(1).step_by(2).sum();

    let ans = alice_score - bob_score;

    println!("{ans}");
}
