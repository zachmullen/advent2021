use std::fs;

fn main() {
    let contents = fs::read_to_string("1.txt").expect("missing input");
    let vals: Vec<u32> = contents
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut count: u32 = 0;
    let mut prev: u32 = 0;
    let iter = vals
        .iter()
        .zip(&vals[1..])
        .zip(&vals[2..])
        .map(|((x, y), z)| (x, y, z));

    for (a, b, c) in iter {
        if prev > 0 && a + b + c > prev {
            count += 1;
        }
        prev = a + b + c;
    }
    println!("{}", count)
}
