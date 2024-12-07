use std::cmp::min;
use colored::Colorize;
use itertools::Itertools;

const QUIZ_INPUT: &str = include_str!("../../data/2024/input4.txt");

fn solve1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let css = lines.iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = lines.len();
    let cols = css[0].len();

    let mut count = 0;
    let mut result = vec![vec!['.'; cols];rows];

    for r in 0..rows {
        for c in 0..cols {
            let found = check(&css, (r, c), (rows, cols));
            count += found;

            if found > 0 {
                result[r][c] = 'X';
            }

        }
    }

    let s = result.iter().map(|r| r.iter().collect::<String>()).join("\n");
    println!("{s}");
    count
}

fn quiz1() -> usize {
    solve1(QUIZ_INPUT)
}

const XMAS: &str = "XMAS";

fn check(css: &Vec<Vec<char>>, (row, col): (usize, usize), (rows, cols): (usize, usize)) -> usize {
    let mut words = vec![];

    if col <= cols - 4 {
        words.push(vec!(css[row][col], css[row][col+1], css[row][col+2], css[row][col+3]).iter().collect::<String>());
    }

    if col >= 3 {
        words.push(vec!(css[row][col], css[row][col-1], css[row][col-2], css[row][col-3]).iter().collect::<String>());
    }

    if row >= 3 {
        words.push(vec!(css[row][col], css[row-1][col], css[row-2][col], css[row-3][col]).iter().collect::<String>());
    }

    if row <= rows - 4 {
        words.push(vec!(css[row][col], css[row+1][col], css[row+2][col], css[row+3][col]).iter().collect::<String>());
    }

    if row <= rows - 4 && col <= cols - 4 {
        words.push(vec!(css[row][col], css[row + 1][col + 1], css[row + 2][col + 2], css[row + 3][col + 3]).iter().collect::<String>());
    }

    if row >= 3 && col <= cols - 4 {
        words.push(vec!(css[row][col], css[row-1][col+1], css[row-2][col+2], css[row-3][col+3]).iter().collect::<String>());
    }

    if row <= rows - 4 && col >= 3 {
        words.push(vec!(css[row][col], css[row+1][col-1], css[row+2][col-2], css[row+3][col-3]).iter().collect::<String>());
    }

    if row >= 3 && col >= 3 {
        words.push(vec!(css[row][col], css[row-1][col-1], css[row-2][col-2], css[row-3][col-3]).iter().collect::<String>());
    };

    words.iter().filter(|&word| word == XMAS).count()

}

const MAS: &str = "MAS";

fn check_x_mas(css: &Vec<Vec<char>>, (row, col): (usize, usize), (rows, cols): (usize, usize)) -> bool {

    let r1 = vec!(css[row-1][col-1], css[row][col], css[row+1][col+1]);
    let r2 = vec!(css[row+1][col-1], css[row][col], css[row-1][col+1]);

    (r1.iter().collect::<String>() == MAS || r1.iter().rev().collect::<String>() == MAS) &&
        (r2.iter().collect::<String>() == MAS || r2.iter().rev().collect::<String>() == MAS)
}

fn solve2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let css = lines.iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = lines.len();
    let cols = css[0].len();

    let mut count = 0;
    let mut result = vec![vec!['.'; cols];rows];

    for r in 1..(rows-1) {
        for c in 1..(cols-1) {
            if css[r][c] == 'A' {
                if check_x_mas(&css, (r, c), (rows, cols)) {
                    count += 1;
                    result[r][c] = 'A';
                }
            }
        }
    }

    let s = result.iter().map(|r| r.iter().collect::<String>()).join("\n");
    println!("{s}");
    count
}

fn quiz2() -> usize {
    solve2(QUIZ_INPUT)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test1() {
        assert_eq!(solve1(SAMPLE), 18);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 2583);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(SAMPLE), 9);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 1978);
    }
}
