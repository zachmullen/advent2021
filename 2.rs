use std::fs;

fn main() {
    let contents = fs::read_to_string("2.txt").expect("missing input");
    let vals: Vec<(&str, i64)> = contents
        .lines()
        .map(|s: &str| s.trim_end().split_ascii_whitespace().collect())
        .map(|s: Vec<&str>| (s[0], s[1].parse::<i64>().unwrap()))
        .collect();

    let (mut aim, mut x, mut y) = (0i64, 0i64, 0i64);
    for (dir, amt) in vals {
        if dir == "forward" {
            x += amt;
            y += aim * amt;
        } else if dir == "down" {
            aim += amt
        } else {
            aim -= amt
        }
    }
    println!("{}", x * y)
}
