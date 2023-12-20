use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../data/2023/input16.txt");

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

type Beam = (Dir, usize, usize);

fn next(map: &Vec<Vec<char>>, (dir, row, col): &Beam) -> Vec<Beam> {
    let symbol = map[*row][*col];

    let row = *row as i32;
    let col = *col as i32;

    let nexts = if symbol == '.' {
        match dir {
            Dir::North => vec![(*dir, row - 1, col)],
            Dir::East => vec![(*dir, row, col + 1)],
            Dir::South => vec![(*dir, row + 1, col)],
            Dir::West => vec![(*dir, row, col - 1)],
        }
    } else if symbol == '|' {
        match dir {
            Dir::North => vec![(*dir, row - 1, col)],
            Dir::East => vec![(Dir::North, row - 1, col), (Dir::South, row + 1, col)],
            Dir::South => vec![(*dir, row + 1, col)],
            Dir::West => vec![(Dir::North, row - 1, col), (Dir::South, row + 1, col)],
        }
    } else if symbol == '-' {
        match dir {
            Dir::North => vec![(Dir::West, row, col - 1), (Dir::East, row, col + 1)],
            Dir::East => vec![(*dir, row, col + 1)],
            Dir::South => vec![(Dir::West, row, col - 1), (Dir::East, row, col + 1)],
            Dir::West => vec![(*dir, row, col - 1)],
        }
    } else if symbol == '/' {
        match dir {
            Dir::North => vec![(Dir::East, row, col + 1)],
            Dir::East => vec![(Dir::North, row - 1, col)],
            Dir::South => vec![(Dir::West, row, col - 1)],
            Dir::West => vec![(Dir::South, row + 1, col)],
        }
    } else if symbol == '\\' {
        match dir {
            Dir::North => vec![(Dir::West, row, col - 1)],
            Dir::East => vec![(Dir::South, row + 1, col)],
            Dir::South => vec![(Dir::East, row, col + 1)],
            Dir::West => vec![(Dir::North, row - 1, col)],
        }
    } else {
        panic!("Invalid symbol: {}", symbol);
    };

    nexts
        .iter()
        .filter(|(_, r, c)| {
            (*r >= 0 && *r < map.len() as i32) && (*c >= 0 && *c < map[0].len() as i32)
        })
        .map(|(d, r, c)| (*d, *r as usize, *c as usize))
        .collect()
}

fn energize(map: &Vec<Vec<char>>, start: Beam) -> HashSet<Beam> {
    let mut energized: HashSet<Beam> = HashSet::new();
    let mut queue: VecDeque<Beam> = VecDeque::new();

    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        if energized.contains(&v) {
            continue;
        }

        energized.insert(v);
        next(map, &v).into_iter().for_each(|b| queue.push_back(b));
    }

    energized
}

fn solve1(data: &str) -> usize {
    let map: Vec<Vec<char>> = load(data);

    let energized: HashSet<Beam> = energize(&map, (Dir::East, 0, 0));

    tiles(energized).len()
}

fn tiles(energized: HashSet<Beam>) -> HashSet<(usize, usize)> {
    energized
        .iter()
        .map(|(_, r, c)| (*r, *c))
        .collect::<HashSet<_>>()
}

fn load(data: &str) -> Vec<Vec<char>> {
    data.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn solve2(data: &str) -> usize {
    let map: Vec<Vec<char>> = load(data);

    let mut max_tiles = 0;
    let mut max_dir = Dir::East;
    let mut max_row = 0;
    let mut max_col = 0;
    let dirs = vec![Dir::East, Dir::South, Dir::North, Dir::West];

    let col = 0;
    for row in 0..map.len() {
        for dir in dirs.iter() {
            let energized: HashSet<Beam> = energize(&map, (*dir, row, col));
            let t = tiles(energized).len();
            if t > max_tiles {
                max_tiles = t;
                max_dir = *dir;
                max_row = row;
                max_col = col;
            }
        }
    }

    let col = map[0].len() - 1;
    for row in 0..map.len() {
        for dir in dirs.iter() {
            let energized: HashSet<Beam> = energize(&map, (*dir, row, col));
            let t = tiles(energized).len();
            if t > max_tiles {
                max_tiles = t;
                max_dir = *dir;
                max_row = row;
                max_col = col;
            }
        }
    }

    let row = 0;
    for col in 1..=map[0].len() - 1 {
        for dir in dirs.iter() {
            let energized: HashSet<Beam> = energize(&map, (*dir, row, col));
            let t = tiles(energized).len();
            if t > max_tiles {
                max_tiles = t;
                max_dir = *dir;
                max_row = row;
                max_col = col;
            }
        }
    }

    let row = map.len() - 1;
    for col in 1..=map[0].len() - 1 {
        for dir in dirs.iter() {
            let energized: HashSet<Beam> = energize(&map, (*dir, row, col));
            let t = tiles(energized).len();
            if t > max_tiles {
                max_tiles = t;
                max_dir = *dir;
                max_row = row;
                max_col = col;
            }
        }
    }

    println!("{:?} {max_row}/{max_col}  {max_tiles}", max_dir);

    max_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_next() {
        let map = load(EXAMPLE);
        assert_eq!(next(&map, &(Dir::East, 0, 0)), vec![(Dir::East, 0, 1)]);
        assert_eq!(next(&map, &(Dir::East, 0, 1)), vec![(Dir::South, 1, 1)]);
        assert_eq!(next(&map, &(Dir::East, 0, 5)), vec![(Dir::South, 1, 5)]);
        assert_eq!(
            next(&map, &(Dir::East, 2, 5)),
            vec![(Dir::North, 1, 5), (Dir::South, 3, 5)]
        );
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 46);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 7307);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE), 51);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 7635);
    }
}
