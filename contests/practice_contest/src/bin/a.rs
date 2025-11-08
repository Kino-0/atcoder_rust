// A - Welcome to AtCoder
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let stdin = stdin.split_whitespace();
    let result: i32 = stdin
        .clone()
        .take(3)
        .map(|n| n.parse::<i32>().unwrap())
        .sum();
    let text = stdin.last().unwrap();
    println!("{result} {text}");
}
