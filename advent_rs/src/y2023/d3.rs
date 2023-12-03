use std::cmp::{max, min};
use std::collections::HashSet;

const INPUT: &str = include_str!("../../data/2023/input3.txt");

fn quiz1() -> u32 {
    solve1(INPUT)
}

fn quiz2() -> u32 {
    solve2(INPUT)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Sch {
    symbols: Vec<(char, Pos)>,
    numbers: Vec<(u32, Pos, Pos)>,
    rows: usize,
}

fn load_sch(data: &str) -> Sch {
    let mut symbols = vec![];
    let mut numbers = vec![];
    let mut rows = 0;

    for (row, line) in data.lines().enumerate() {
        let mut nums: Vec<(char, usize)> = vec![];
        let mut cols = line.chars().count();

        for (col, c) in line.chars().enumerate() {
            let mut is_numeric = false;
            if c.is_numeric() {
                nums.push((c, col));
                is_numeric = true;
            } else if c != '.' {
                symbols.push((c, Pos(row, col)))
            }

            if !is_numeric || col == cols - 1 {
                if !nums.is_empty() {
                    let start = nums[0].1;
                    let end = nums[nums.len() - 1].1;

                    let num = nums
                        .iter()
                        .map(|(n, _)| n)
                        .fold(0_u32, |acc, &v| acc * 10 + v.to_digit(10).unwrap());

                    numbers.push((num, Pos(row, start), Pos(row, end)));

                    nums.clear();
                }
            }
        }
        rows = row + 1;
    }

    Sch {
        symbols,
        numbers,
        rows,
    }
}

fn solve1(data: &str) -> u32 {
    let sch = load_sch(data);

    let set = sch
        .symbols
        .iter()
        .flat_map(|(_, sp)| find_adjacency_numbers(sp, &sch.numbers, sch.rows))
        .collect::<HashSet<_>>();

    set.iter().map(|(n, _, _)| *n).sum()
}

fn solve2(data: &str) -> u32 {
    let sch = load_sch(data);
    sch.symbols
        .iter()
        .map(|(_, sp)| find_adjacency_numbers(sp, &sch.numbers, sch.rows))
        .filter(|vs| vs.len() == 2)
        .map(|vs| {
            vs.iter()
                .map(|(n, _, _)| *n)
                .reduce(|acc, v| acc * v)
                .unwrap()
        })
        .sum()
}

fn find_adjacency_numbers(
    Pos(row, col): &Pos,
    numbers: &Vec<(u32, Pos, Pos)>,
    rows: usize,
) -> Vec<(u32, Pos, Pos)> {
    numbers
        .iter()
        .filter_map(|(num, Pos(r, sc), Pos(_, ec))| {
            if *r == *row {
                if (*col != 0 && *ec == *col - 1) || *sc == *col + 1 {
                    Some((*num, Pos(*r, *sc), Pos(*r, *ec)))
                } else {
                    None
                }
            } else if (*r != 0 && *r - 1 == *row) || (*r != rows - 1 && *r + 1 == *row) {
                if *sc <= *col + 1 && (*col != 0 && *ec >= *col - 1) {
                    Some((*num, Pos(*r, *sc), Pos(*r, *ec)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const SAMPLE2: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.581
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_load_sch1() {
        let sch = load_sch(SAMPLE2);

        let expected = Sch {
            symbols: vec![
                ('*', Pos(1, 3)),
                ('#', Pos(3, 6)),
                ('*', Pos(4, 3)),
                ('+', Pos(5, 5)),
                ('$', Pos(8, 3)),
                ('*', Pos(8, 5)),
            ],

            numbers: vec![
                (467, Pos(0, 0), Pos(0, 2)),
                (114, Pos(0, 5), Pos(0, 7)),
                (35, Pos(2, 2), Pos(2, 3)),
                (633, Pos(2, 6), Pos(2, 8)),
                (617, Pos(4, 0), Pos(4, 2)),
                (581, Pos(5, 7), Pos(5, 9)),
                (592, Pos(6, 2), Pos(6, 4)),
                (755, Pos(7, 6), Pos(7, 8)),
                (664, Pos(9, 1), Pos(9, 3)),
                (598, Pos(9, 5), Pos(9, 7)),
            ],

            rows: 10,
        };

        assert_eq!(sch, expected);
    }

    #[test]
    fn test_find_adjacency_numbers() {
        let nums = vec![
            (467, Pos(0, 0), Pos(0, 2)),
            (114, Pos(0, 5), Pos(0, 7)),
            (35, Pos(2, 2), Pos(2, 3)),
            (633, Pos(2, 6), Pos(2, 8)),
            (617, Pos(4, 0), Pos(4, 2)),
            (58, Pos(5, 7), Pos(5, 8)),
            (592, Pos(6, 2), Pos(6, 4)),
            (755, Pos(7, 6), Pos(7, 8)),
            (664, Pos(9, 1), Pos(9, 3)),
            (598, Pos(9, 5), Pos(9, 7)),
        ];

        assert_eq!(
            find_adjacency_numbers(&Pos(1, 3), &nums, 10,),
            vec![(467, Pos(0, 0), Pos(0, 2)), (35, Pos(2, 2), Pos(2, 3))]
        );

        assert_eq!(
            find_adjacency_numbers(&Pos(0, 3), &nums, 10,),
            vec![(467, Pos(0, 0), Pos(0, 2))]
        );

        assert_eq!(
            find_adjacency_numbers(&Pos(9, 4), &nums, 10,),
            vec![(664, Pos(9, 1), Pos(9, 3)), (598, Pos(9, 5), Pos(9, 7)),]
        );
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 4361);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(), 520019);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 467835);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 75519888);
    }
}
