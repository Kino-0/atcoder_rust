// 001 - Yokan Party（★4）
// https://atcoder.jp/contests/typical90/tasks/typical90_a

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("パース失敗"));

    let _n = tokens.next().expect("N取得失敗"); // 切れ目の総数
    let l = tokens.next().expect("L取得失敗"); // 全体の長さ
    let k = tokens.next().expect("K取得失敗"); // 選択する切れ目の数
    let a: Vec<usize> = tokens.collect(); // 切れ目の位置

    let mut a_cm = Vec::new();
    let mut mem = a[0];
    for th_pice in 1..=k {
        for i in &a {
            if th_pice * (l / (1 + k)) < *i {
                a_cm.push(mem);
                break;
            }
            mem = *i;
        }
    }

    let ans = a_cm
        .iter()
        .scan(0, |prev, &x| {
            let diff = x - *prev;
            *prev = x;
            Some(diff)
        })
        .min()
        .expect("スコアの計算に失敗");

    println!("{ans}");
}
