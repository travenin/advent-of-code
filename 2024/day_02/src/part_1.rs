fn main() {
    let input = include_str!("../input.txt");

    let mut safe_sum: u64 = 0;
    'line: for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut last_diff: i64 = 0;
        for pair in numbers.windows(2) {
            if let [a, b] = pair {
                let diff = a - b;

                if last_diff != 0 && last_diff.signum() != diff.signum() {
                    continue 'line;
                }
                last_diff = diff;

                if diff == 0 || diff.abs() > 3 {
                    continue 'line;
                }
            }
        }
        safe_sum += 1;
    }

    println!("{}", safe_sum);
}
