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
    fn test_solve1() {
        assert_eq!(solve1(EX), 405);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 35360);
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
