// ABC083B - Some Sums

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut input = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let a = input.next().unwrap();
    let b = input.next().unwrap();

    let mut ans: Vec<i32> = Vec::new();

    for i in 1..=n {
        let mut numbers: Vec<i32> = Vec::new();
        for j in 1..=i.ilog10() + 1 {
            numbers.push((i % 10_i32.pow(j)) / (10_i32.pow(j - 1)));
        }
        let sum = numbers.iter().sum();
        if a <= sum && sum <= b {
            ans.push(i);
        }
    }

    let ans: i32 = ans.iter().sum();
    println!("{ans}");
}
