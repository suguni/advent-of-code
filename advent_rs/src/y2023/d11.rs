use num::abs;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../data/2023/input11.txt");

type Pos = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Image {
    galaxies: Vec<Pos>,
    width: usize,
    height: usize,
}

impl Image {
    fn from(data: &str) -> Self {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();

        let galaxies = data
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(col, c)| (row, col))
            })
            .collect();

        Image {
            galaxies,
            width,
            height,
        }
    }

    fn expand(&mut self, expansion: usize) {
        let rows = self
            .galaxies
            .iter()
            .map(|g| g.0)
            .collect::<HashSet<usize>>();
        let cols = self
            .galaxies
            .iter()
            .map(|g| g.1)
            .collect::<HashSet<usize>>();

        let mut row_expands = vec![0; self.height];
        let mut ex = 0;
        for r in 0..self.height {
            if !rows.contains(&r) {
                ex += expansion;
            }
            row_expands[r] = ex;
        }
        self.height += ex;

        let mut col_expands = vec![0; self.width];
        let mut ex = 0;
        for c in 0..self.width {
            if !cols.contains(&c) {
                ex += expansion;
            }
            col_expands[c] = ex;
        }
        self.width += ex;

        for (row, col) in self.galaxies.iter_mut() {
            *row += row_expands[*row];
            *col += col_expands[*col];
        }
    }

    fn sum_of_shortest_path(&self) -> u64 {
        let mut lens = vec![];

        for i in 0..self.galaxies.len() {
            for j in i..self.galaxies.len() {
                if i != j {
                    let (r1, c1) = self.galaxies[i];
                    let (r2, c2) = self.galaxies[j];
                    lens.push(
                        abs(r2 as i64 - r1 as i64) as u64 + abs(c2 as i64 - c1 as i64) as u64,
                    );
                }
            }
        }

        lens.iter().sum::<u64>()
    }
}

fn solve1(data: &str) -> u64 {
    let mut image = Image::from(data);
    image.expand(1);
    image.sum_of_shortest_path()
}

fn solve2(data: &str, expansion: usize) -> u64 {
    let mut image = Image::from(data);
    image.expand(expansion);
    image.sum_of_shortest_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE), 374);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 10228230);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE, 1), 374);
        assert_eq!(solve2(EXAMPLE, 10 - 1), 1030);
        assert_eq!(solve2(EXAMPLE, 100 - 1), 8410);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT, 1000000 - 1), 447073334102);
    }

    #[test]
    fn test_path_len() {
        let image = Image {
            width: 13,
            height: 12,
            galaxies: vec![
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5),
            ],
        };

        assert_eq!(image.sum_of_shortest_path(), 374);
    }

    #[test]
    fn test_expand() {
        let mut image = Image {
            width: 10,
            height: 10,
            galaxies: vec![
                (0, 3),
                (1, 7),
                (2, 0),
                (4, 6),
                (5, 1),
                (6, 9),
                (8, 7),
                (9, 0),
                (9, 4),
            ],
        };

        image.expand(1);

        assert_eq!(
            image,
            Image {
                width: 13,
                height: 12,
                galaxies: vec![
                    (0, 4),
                    (1, 9),
                    (2, 0),
                    (5, 8),
                    (6, 1),
                    (7, 12),
                    (10, 9),
                    (11, 0),
                    (11, 5)
                ]
            }
        );
    }

    #[test]
    fn test_load() {
        let image = Image::from(EXAMPLE);

        assert_eq!(
            image,
            Image {
                width: 10,
                height: 10,
                galaxies: vec![
                    (0, 3),
                    (1, 7),
                    (2, 0),
                    (4, 6),
                    (5, 1),
                    (6, 9),
                    (8, 7),
                    (9, 0),
                    (9, 4)
                ]
            }
        );
    }
}
