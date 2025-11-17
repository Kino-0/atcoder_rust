// ABC086C - Traveling

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("標準入力読込失敗");
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse::<isize>().expect("パース失敗"));
    let n = input.next().unwrap();
    let mut from_t = 0;
    let mut from_x = 0;
    let mut from_y = 0;

    for _ in 1..=n {
        let to_t = input.next().unwrap();
        let to_x = input.next().unwrap();
        let to_y = input.next().unwrap();
        let times = to_t - from_t;
        let distance = (to_x - from_x).abs() + (to_y - from_y).abs();

        if (times - distance).is_negative() || times % 2 != distance % 2 {
            println!("No");
            return;
        }

        from_t = to_t;
        from_x = to_x;
        from_y = to_y;
    }
    println!("Yes");
}
