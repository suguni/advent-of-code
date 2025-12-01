use std::collections::HashMap;

const INPUT: &str = include_str!("../../data/2023/input14.txt");

struct Block {
    chars: Vec<Vec<char>>,
    col_size: usize,
    row_size: usize,
    sharp_columns: HashMap<usize, Vec<usize>>,
    sharp_rows: HashMap<usize, Vec<usize>>,
}

fn load(data: &str) -> (Block, Vec<Vec<usize>>) {
    let row_size = data.lines().count();
    let col_size = data.lines().next().unwrap().chars().count();

    let ss: Vec<char> = data
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut sharp_rows = HashMap::new();
    let mut sharp_columns = HashMap::new();

    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, key)| *key == '#')
                .map(|(col, _)| (row, col))
                .collect::<Vec<_>>()
        })
        .for_each(|(r, c)| {
            sharp_rows
                .entry(r)
                .and_modify(|vs: &mut Vec<_>| vs.push(c))
                .or_insert(vec![c]);
            sharp_columns
                .entry(c)
                .and_modify(|vs: &mut Vec<_>| vs.push(r))
                .or_insert(vec![r]);
        });

    let columns: Vec<Vec<char>> = (0..col_size)
        .map(|c| (0..row_size).map(|r| ss[r * col_size + c]).collect())
        .collect();

    let rounded_columns: Vec<Vec<usize>> = (0..col_size)
        .map(|c| {
            (0..row_size)
                .filter_map(|r| {
                    if ss[r * col_size + c] == 'O' {
                        Some(r)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    (
        Block {
            chars: columns,
            col_size,
            row_size,
            sharp_columns,
            sharp_rows,
        },
        rounded_columns,
    )
}

fn tilted(line: &Vec<char>) -> Vec<usize> {
    let mut result = vec![];

    let mut start = 0;
    let mut count = 0;

    for i in 0..line.len() {
        if line[i] == '#' {
            for c in 0..count {
                result.push(line.len() - start - c);
            }
            start = i + 1;
            count = 0;
        } else if line[i] == 'O' {
            count += 1;
        }
    }

    for c in 0..count {
        result.push(line.len() - start - c);
    }

    result
}

fn tilted2(rounded: &Vec<usize>, sharp: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];

    if rounded.is_empty() {
    } else if sharp.is_empty() {
        for c in 0..rounded.len() {
            result.push(c);
        }
    } else {
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;
        let mut base = 0;

        loop {
            let r = rounded[i];
            let s = sharp[j];

            if r < s {
                result.push(count + base);
                count += 1;
                i += 1;
            } else {
                base = s + 1;
                count = 0;
                j += 1;
            }

            if i > rounded.len() - 1 {
                break;
            }

            if j > sharp.len() - 1 {
                for n in 0..(rounded.len() - i) {
                    result.push(n + base);
                }
                break;
            }
        }
    }

    result
}

fn reverse_tilted2(rounded: &Vec<usize>, sharp: &Vec<usize>, length: usize) -> Vec<usize> {
    let rounded = rounded.iter().map(|p| length - p - 1).rev().collect();
    let sharp = sharp.iter().map(|p| length - p - 1).rev().collect();
    let tilted = tilted2(&rounded, &sharp);
    tilted.iter().map(|p| length - p - 1).rev().collect()
}

fn solve1(data: &str) -> usize {
    let (block, rounded_columns) = load(data);
    north_load(
        &tilt2_all(rounded_columns, &block.sharp_columns, block.row_size, true),
        block.row_size,
    )
}

fn north_load(platform: &Vec<Vec<usize>>, length: usize) -> usize {
    platform
        .iter()
        .map(|vs| vs.iter().map(|p| length - p).sum::<usize>())
        .sum()
}

fn transpose(rows: Vec<Vec<usize>>, length: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for c in 0..length {
        result.push(
            rows.iter()
                .enumerate()
                .filter_map(|(r, row)| if row.contains(&c) { Some(r) } else { None })
                .collect::<Vec<_>>(),
        );
    }
    result
}

fn solve2(data: &str, count: usize) -> usize {
    let (block, mut platform) = load(data);

    for i in 0..count {
        let tiled = tilt2_all(platform, &block.sharp_columns, block.row_size, true);
        platform = transpose(tiled, block.row_size);
        let tiled = tilt2_all(platform, &block.sharp_rows, block.col_size, true);
        platform = transpose(tiled, block.col_size);
        let tiled = tilt2_all(platform, &block.sharp_columns, block.row_size, false);
        platform = transpose(tiled, block.row_size);
        let tiled = tilt2_all(platform, &block.sharp_rows, block.col_size, false);
        platform = transpose(tiled, block.col_size);
    }

    let result = north_load(&platform, block.row_size);

    print(
        &transpose(platform, block.row_size),
        &block.sharp_rows,
        block.col_size,
    );

    result
}

fn print(platform: &Vec<Vec<usize>>, sharp_rows: &HashMap<usize, Vec<usize>>, length: usize) {
    platform.iter().enumerate().for_each(|(r, row)| {
        let vec1 = vec![];
        let sharps = sharp_rows.get(&r).unwrap_or(&vec1);

        (0..length).for_each(|c| {
            if row.contains(&c) {
                print!("O");
            } else if sharps.contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    })
}

fn tilt2_all(
    positions: Vec<Vec<usize>>,
    sharps: &HashMap<usize, Vec<usize>>,
    length: usize,
    dir: bool,
) -> Vec<Vec<usize>> {
    positions
        .iter()
        .enumerate()
        .map(|(col, rounded)| {
            let def = Vec::new();
            let sharp = sharps.get(&col).unwrap_or(&def);
            if dir {
                tilted2(rounded, sharp)
            } else {
                reverse_tilted2(rounded, sharp, length)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 136);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE, 100000), 64);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 105461);
    }

    #[test]
    fn test_tilted() {
        assert_eq!(
            tilted(&vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#']),
            vec![10, 9, 8, 7]
        );
        assert_eq!(
            tilted(&vec!['#', '.', '#', 'O', '.', '.', '.', '.', '#', '.']),
            vec![7]
        );

        assert_eq!(
            tilted(&vec!['.', '#', 'O', '.', '.', '#', 'O', '.', '#', '.']),
            vec![8, 4]
        );
    }

    #[test]
    fn test_tilted2() {
        assert_eq!(tilted2(&vec![0, 1, 3, 5], &vec![8, 9]), vec![0, 1, 2, 3]);
        assert_eq!(tilted2(&vec![3, 4, 9], &vec![]), vec![0, 1, 2]);
        assert_eq!(tilted2(&vec![1], &vec![3]), vec![0]);
        assert_eq!(tilted2(&vec![5], &vec![0, 2, 6, 8, 9]), vec![3]);
        assert_eq!(tilted2(&vec![3, 6], &vec![1, 5]), vec![2, 6]);
    }

    #[test]
    fn test_reverse_tilted2() {
        assert_eq!(
            reverse_tilted2(&vec![0, 1, 3, 5], &vec![8, 9], 10),
            vec![4, 5, 6, 7]
        );
    }

    #[test]
    fn test_load() {
        let (block, _) = load(EXAMPLE);
        assert_eq!(block.row_size, 10);
        assert_eq!(block.col_size, 10);
        assert_eq!(
            block.chars[0],
            vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#']
        );

        assert_eq!(block.sharp_columns[&0], vec![8, 9]);
        assert_eq!(block.sharp_rows[&0], vec![5]);
    }
}
