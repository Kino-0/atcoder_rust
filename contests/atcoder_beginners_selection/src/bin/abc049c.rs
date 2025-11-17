// ABC049C - 白昼夢

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");

    let ans = input
        .replace("eraser", "")
        .replace("erase", "")
        .replace("dreamer", "")
        .replace("dream", "")
        .chars()
        .filter(|x| !x.is_whitespace())
        .count();

    if ans == 0 {
        println!("YES")
    } else {
        println!("NO")
    }
}
