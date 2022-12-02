use crate::read_file;

fn total_score(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let inputs = line.chars().map(|c| c).collect::<Vec<char>>();

            // A Rock, B Paper, C Scissors
            // X Rock, Y Paper, Z Scissors
            match (inputs[0], inputs[2]) {
                ('A', 'X') => 3 + 1,
                ('B', 'X') => 0 + 1,
                ('C', 'X') => 6 + 1,
                ('A', 'Y') => 6 + 2,
                ('B', 'Y') => 3 + 2,
                ('C', 'Y') => 0 + 2,
                ('A', 'Z') => 0 + 3,
                ('B', 'Z') => 6 + 3,
                ('C', 'Z') => 3 + 3,
                _ => unreachable!(),
            }
        })
        .sum()
}

fn quiz1() -> i32 {
    let input = read_file("data/2022/input2.txt");
    total_score(&input)
}

fn round_score(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let inputs = line.chars().map(|c| c).collect::<Vec<char>>();

            //    1        2        3
            // A Rock, B Paper, C Scissors
            // X lose, Y draw, Z win
            match (inputs[0], inputs[2]) {
                ('A', 'X') => 3 + 0, // scissors
                ('B', 'X') => 1 + 0, // rock
                ('C', 'X') => 2 + 0, // paper
                ('A', 'Y') => 1 + 3, // rock
                ('B', 'Y') => 2 + 3, // paper
                ('C', 'Y') => 3 + 3, // scissors
                ('A', 'Z') => 2 + 6, // paper
                ('B', 'Z') => 3 + 6, // scissors
                ('C', 'Z') => 1 + 6, // rock
                _ => unreachable!(),
            }
        })
        .sum()
}

fn quiz2() -> i32 {
    let input = read_file("data/2022/input2.txt");
    round_score(&input)
}

#[test]
fn test_total_score() {
    let input = "A Y
B X
C Z";
    assert_eq!(total_score(input), 15);
}

#[test]
fn test_quiz1() {
    assert_eq!(quiz1(), 10310);
}

#[test]
fn test_round_score() {
    let input = "A Y
B X
C Z";
    assert_eq!(round_score(input), 12);
}

#[test]
fn test_quiz2() {
    assert_eq!(quiz2(), 14859);
}
