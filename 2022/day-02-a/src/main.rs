const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOST: u32 = 0;

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn char_to_shape(char: &str) -> Shape {
    match char {
        "A" => Shape::Rock,
        "X" => Shape::Rock,

        "B" => Shape::Paper,
        "Y" => Shape::Paper,

        "C" => Shape::Scissors,
        "Z" => Shape::Scissors,

        other => panic!("Unexpected input character '{}'", other),
    }
}

fn game_points(ours: Shape, theirs: Shape) -> u32 {
    match ours {
        Shape::Rock => match theirs {
            Shape::Rock => DRAW,
            Shape::Paper => LOST,
            Shape::Scissors => WIN,
        },
        Shape::Paper => match theirs {
            Shape::Rock => WIN,
            Shape::Paper => DRAW,
            Shape::Scissors => LOST,
        },
        Shape::Scissors => match theirs {
            Shape::Rock => LOST,
            Shape::Paper => WIN,
            Shape::Scissors => DRAW,
        },
    }
}

fn shape_points(shape: Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn round_points(round: &str) -> u32 {
    let splits = round.split(" ").collect::<Vec<&str>>();
    let theirs = char_to_shape(splits[0]);
    let ours = char_to_shape(splits[1]);

    shape_points(ours) + game_points(ours, theirs)
}

fn main() {
    let input = include_str!("../input.txt");
    let rounds = input.lines().collect::<Vec<&str>>();

    let sum: u32 = rounds.iter().map(|r| round_points(r)).sum();
    println!("{}", sum);
}
