fn main() {
    let input = include_str!("../input.txt");

    let sums = input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    println!("{}", sums.iter().max().unwrap());
}
