fn main() {
    let input = include_str!("../input.txt");

    let mut left = Vec::<i64>::new();
    let mut right = Vec::<i64>::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i64>().unwrap());
        right.push(parts[1].parse::<i64>().unwrap());
    }
    left.sort();
    right.sort();

    let sum: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{}", sum);
}