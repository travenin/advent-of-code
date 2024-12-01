fn main() {
    let input = include_str!("../input.txt");

    let mut sums = input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));

    println!("{:?}", &sums[0..=2].iter().sum::<u32>());
}
