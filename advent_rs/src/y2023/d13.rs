use num::abs;
use std::cmp::min;

const INPUT: &str = include_str!("../../data/2023/input13.txt");

struct Block {
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

fn load_block(input: &str) -> Block {
    let rows: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
                .collect()
        })
        .collect();

    let cols_count = input.lines().next().unwrap().chars().count();

    let mut cols: Vec<Vec<usize>> = vec![vec![]; cols_count];
    for (rx, row) in rows.iter().enumerate() {
        for col in row {
            cols.get_mut(*col).unwrap().push(rx);
        }
    }

    Block { rows, cols }
}

fn load(input: &str) -> Vec<Block> {
    input
        .split("\n\n")
        .into_iter()
        .map(|block| load_block(block))
        .collect()
}

fn find_reflect(series: Vec<Vec<usize>>) -> Option<usize> {
    let candidates = series
        .windows(2)
        .enumerate()
        .filter_map(|(idx, vs)| if vs[0] == vs[1] { Some(idx) } else { None })
        .collect::<Vec<usize>>();

    for start in candidates {
        let len = min(start + 1, series.len() - start - 1);
        let mut found = true;

        for s in 1..len {
            if series[start - s] != series[start + 1 + s] {
                found = false;
                break;
            }
        }

        if found {
            return Some(start);
        }
    }

    None
}

fn solve1(input: &str) -> usize {
    load(input)
        .into_iter()
        .map(|block| {
            let rows = block.rows;
            let cols = block.cols;
            find_reflect(rows)
                .map(|r| (r + 1) * 100)
                .unwrap_or_else(|| find_reflect(cols).unwrap() + 1)
        })
        .sum()
}

fn is_smudged(line1: usize, line2: usize) -> bool {
    check_bit(line1 ^ line2)
}

fn vs2bin(vs: &Vec<usize>) -> usize {
    vs.iter()
        .fold(0_usize, |acc, v| acc + 2_usize.pow(*v as u32))
}

fn find_reflect_with_smudge(series: Vec<Vec<usize>>) -> Option<usize> {
    let series = series.iter().map(|vs| vs2bin(vs)).collect::<Vec<usize>>();

    let candidates = series
        .windows(2)
        .enumerate()
        .filter_map(|(idx, vs)| {
            if vs[0] == vs[1] {
                Some((idx, 0))
            } else if is_smudged(vs[0], vs[1]) {
                Some((idx, 1))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, usize)>>();

    for (start, mut smudge_count) in candidates {
        let len = min(start + 1, series.len() - start - 1);
        let mut found = true;

        for s in 1..len {
            let l1 = series[start - s];
            let l2 = series[start + 1 + s];
            if l1 != l2 {
                if is_smudged(l1, l2) {
                    smudge_count += 1;
                    if smudge_count > 1 {
                        break;
                    }
                } else {
                    smudge_count = 0;
                    break;
                }
            }
        }

        if smudge_count == 1 {
            return Some(start);
        }
    }

    None
}

fn check_bit(bits: usize) -> bool {
    bits != 0 && (bits & (bits - 1)) == 0
}

fn solve2(input: &str) -> usize {
    load(input)
        .into_iter()
        .map(|block| {
            let rows = block.rows;
            let cols = block.cols;
            find_reflect_with_smudge(rows)
                .map(|r| (r + 1) * 100)
                .unwrap_or_else(|| find_reflect_with_smudge(cols).unwrap() + 1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    const EX_V: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const EX_H: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_find_reflect_with_smudge() {
        assert_eq!(find_reflect_with_smudge(ex_v_cols()), None);
        assert_eq!(find_reflect_with_smudge(ex_v_rows()), Some(2));

        assert_eq!(find_reflect_with_smudge(ex_h_cols()), None);
        assert_eq!(find_reflect_with_smudge(ex_h_rows()), Some(0));
    }

    #[test]
    fn test_check_bit() {
        assert_eq!(check_bit(0b100), true);
        assert_eq!(check_bit(0b101), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(0b101100110 ^ 0b001100110, 0b100000000);
    }

    #[test]
    fn test_eq_or_smudge() {
        assert!(is_smudged(
            vs2bin(&vec![1, 2, 3]),
            vs2bin(&vec![1, 2, 3, 4])
        ));

        assert!(is_smudged(
            vs2bin(&vec![0, 1, 2, 3]),
            vs2bin(&vec![1, 2, 3])
        ));

        assert_eq!(
            is_smudged(vs2bin(&vec![1, 2, 3]), vs2bin(&vec![1, 2, 3])),
            false
        );

        assert_eq!(
            is_smudged(vs2bin(&vec![0, 1, 3]), vs2bin(&vec![1, 2, 3])),
            false
        );

        assert_eq!(
            is_smudged(vs2bin(&vec![1, 3]), vs2bin(&vec![1, 2, 3, 4])),
            false
        );
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EX), 405);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 35360);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EX), 400);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 36755);
    }

    #[test]
    fn test_load_block() {
        let block_v = load_block(EX_V);
        assert_eq!(block_v.rows, ex_v_rows());
        assert_eq!(block_v.cols, ex_v_cols());

        let block_h = load_block(EX_H);
        assert_eq!(block_h.rows, ex_h_rows());
        assert_eq!(block_h.cols, ex_h_cols());
    }

    fn ex_h_cols() -> Vec<Vec<usize>> {
        vec![
            vec![0, 1, 3, 4, 6],
            vec![3, 4],
            vec![2, 3, 4, 5],
            vec![2, 3, 4, 5],
            vec![0, 3, 4],
            vec![0, 1, 6],
            vec![2, 3, 4, 5],
            vec![2, 3, 4, 5],
            vec![0, 1, 2, 5, 6],
        ]
    }

    fn ex_h_rows() -> Vec<Vec<usize>> {
        vec![
            vec![0, 4, 5, 8],
            vec![0, 5, 8],
            vec![2, 3, 6, 7, 8],
            vec![0, 1, 2, 3, 4, 6, 7],
            vec![0, 1, 2, 3, 4, 6, 7],
            vec![2, 3, 6, 7, 8],
            vec![0, 5, 8],
        ]
    }

    fn ex_v_cols() -> Vec<Vec<usize>> {
        vec![
            vec![0, 2, 3, 6],
            vec![2, 3],
            vec![0, 1, 4, 5, 6],
            vec![0, 5],
            vec![1, 4, 6],
            vec![1, 4, 6],
            vec![0, 5],
            vec![0, 1, 4, 5, 6],
            vec![2, 3],
        ]
    }

    fn ex_v_rows() -> Vec<Vec<usize>> {
        vec![
            vec![0, 2, 3, 6, 7],
            vec![2, 4, 5, 7],
            vec![0, 1, 8],
            vec![0, 1, 8],
            vec![2, 4, 5, 7],
            vec![2, 3, 6, 7],
            vec![0, 2, 4, 5, 7],
        ]
    }

    #[test]
    fn test_vertical() {
        assert_eq!(find_reflect(ex_v_cols()), Some(4));
        assert_eq!(find_reflect(ex_h_cols()), None);
    }

    #[test]
    fn test_horizontal() {
        assert_eq!(find_reflect(ex_h_rows()), Some(3));
        assert_eq!(find_reflect(ex_v_rows()), None);
    }

    #[test]
    fn test_ok() {
        assert_eq!(1, 1);
    }
}
