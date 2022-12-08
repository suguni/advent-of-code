use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, digit1, newline, not_line_ending, space1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use regex::Regex;

use crate::read_file;

const FILE_NAME: &str = "data/2022/input8.txt";

fn load(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn visible(grid: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
        return true;
    }

    let t = grid[row][col];

    if grid[row][..col].iter().all(|&v| v < t) {
        return true;
    }
    if grid[row][col + 1..].iter().all(|&v| v < t) {
        return true;
    }

    if (0..row).all(|r| grid[r][col] < t) {
        return true;
    }

    if (row + 1..rows).all(|r| grid[r][col] < t) {
        return true;
    }

    false
}

fn proc1(input: &str) -> usize {
    let grid = load(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visible(&grid, (r, c)) {
                count += 1;
            }
        }
    }
    count
}

fn quiz1() -> usize {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn count(grid: &Vec<Vec<u8>>, (r, c): (usize, usize), t: u8) -> (u32, bool) {
    if grid[r][c] >= t {
        (1, true)
    } else {
        (1, false)
    }
}

fn visible_score(grid: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let t = grid[row][col];

    let mut l = 0;
    if col != 0 {
        for c in (0..col).rev() {
            let (inc, brk) = count(grid, (row, c), t);
            l += inc;
            if brk {
                break;
            }
        }
    }

    let mut r = 0;
    if col != cols - 1 {
        for c in (col + 1)..cols {
            let (inc, brk) = count(grid, (row, c), t);
            r += inc;
            if brk {
                break;
            }
        }
    }

    let mut top = 0;
    if row != 0 {
        for r in (0..row).rev() {
            let (inc, brk) = count(grid, (r, col), t);
            top += inc;
            if brk {
                break;
            }
        }
    }

    let mut b = 0;
    if row != rows - 1 {
        for r in (row + 1)..rows {
            let (inc, brk) = count(grid, (r, col), t);
            b += inc;
            if brk {
                break;
            }
        }
    }

    l * r * b * top
}

fn proc2(input: &str) -> u32 {
    let grid = load(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    let mut s = vec![vec![0_u16; cols]; rows];
    let mut max = 0;

    for r in 0..rows {
        for c in 0..cols {
            let v = visible_score(&grid, (r, c));
            if v > max {
                max = v;
            }
        }
    }
    max
}

fn quiz2() -> u32 {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 21);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 1782);
    }

    #[test]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 8);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 474606);
    }
}
