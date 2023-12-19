const INPUT: &str = include_str!("../../data/2023/input14.txt");

struct Block {
    columns: Vec<Vec<char>>,
    col_size: usize,
    row_size: usize,
}

fn load(data: &str) -> Block {
    let row_size = data.lines().count();
    let col_size = data.lines().next().unwrap().chars().count();

    let ss: Vec<char> = data
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let columns: Vec<Vec<char>> = (0..col_size)
        .map(|c| (0..row_size).map(|r| ss[r * col_size + c]).collect())
        .collect();

    Block {
        columns,
        col_size,
        row_size,
    }
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

fn solve1(data: &str) -> usize {
    let block = load(data);
    block.columns.iter().flat_map(|line| tilted(line)).sum()
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
    fn test_load() {
        let block = load(EXAMPLE);
        assert_eq!(block.row_size, 10);
        assert_eq!(block.col_size, 10);
        assert_eq!(
            block.columns[0],
            vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#']
        );
    }
}
