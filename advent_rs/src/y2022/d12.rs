use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{interpolate_color, read_file};

const FILE_NAME: &str = "data/2022/input12.txt";

#[derive(Debug, PartialEq)]
struct Grid {
    grid: Vec<u8>,
    rows: usize,
    cols: usize,
    start: usize,
    target: usize,
}

fn neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<usize> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut indices = Vec::new();
    let center = grid[row][col] as i16;

    if row > 0 {
        let diff = grid[row - 1][col] as i16 - center;
        if diff == 0 || diff == 1 {
            indices.push((row - 1) * cols + col);
        }
    }
    if row < rows - 1 {
        let diff = grid[row + 1][col] as i16 - center;
        if diff == 0 || diff == 1 {
            indices.push((row + 1) * cols + col);
        }
    }
    if col > 0 {
        let diff = grid[row][col - 1] as i16 - center;
        if diff == 0 || diff == 1 {
            indices.push(row * cols + col - 1);
        }
    }
    if col < cols - 1 {
        let diff = grid[row][col + 1] as i16 - center;
        if diff == 0 || diff == 1 {
            indices.push(row * cols + col + 1);
        }
    }
    indices
}

fn load(input: &str) -> Grid {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = map.len();
    let cols = map[0].len();

    let mut grid = vec![0; rows * cols];
    let mut start = 0;
    let mut target = 0;

    let a = 'a' as u8;
    let z = 'z' as u8;

    for (row, line) in map.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            let idx = row * cols + col;
            if cell == 'S' {
                grid[idx] = 0;
                start = idx;
            } else if cell == 'E' {
                grid[idx] = z - a;
                target = idx;
            } else {
                grid[idx] = (cell as u8) - a;
            }
        }
    }

    Grid {
        grid,
        rows,
        cols,
        start,
        target,
    }
}

fn build_graph(grid: &Grid) -> HashMap<usize, Vec<usize>> {
    let map = &grid.grid;
    map.iter()
        .enumerate()
        .map(|(idx, center)| {
            let center = *center as i16;
            let row = idx / grid.cols;
            let col = idx % grid.cols;

            let mut n_idx = vec![];
            if row > 0 {
                n_idx.push((row - 1) * grid.cols + col);
            }
            if row < grid.rows - 1 {
                n_idx.push((row + 1) * grid.cols + col);
            }
            if col > 0 {
                n_idx.push(row * grid.cols + col - 1);
            }
            if col < grid.cols - 1 {
                n_idx.push(row * grid.cols + col + 1);
            }

            (
                idx,
                n_idx
                    .into_iter()
                    .filter(|&idx| map[idx] as i16 - center <= 1)
                    .collect::<_>(),
            )
        })
        .collect()
}

fn proc1(input: &str) -> u32 {
    let grid = load(input);
    println!("{grid:?}");

    let graph = build_graph(&grid);

    let mut dist = vec![u32::MAX; grid.rows * grid.cols];
    dist[grid.start] = 0;

    let mut visited = HashSet::new();
    visited.insert(grid.start);

    let mut next_pos = VecDeque::new();
    push_back_next_nodes(&graph, grid.start, &visited, &mut next_pos);

    println!("{:?}", next_pos);
    while let Some((from, to)) = next_pos.pop_front() {
        let next = dist[from] + 1;
        if dist[to] > next {
            dist[to] = next;
        }

        if visited.contains(&to) {
            continue;
        }

        visited.insert(to);
        push_back_next_nodes(&graph, to, &visited, &mut next_pos);
    }

    draw_map(&dist, grid.rows, grid.cols, grid.start, grid.target);
    dist[grid.target]
}

fn push_back_next_nodes(
    graph: &HashMap<usize, Vec<usize>>,
    current: usize,
    visited: &HashSet<usize>,
    queue: &mut VecDeque<(usize, usize)>,
) {
    if let Some(nodes) = graph.get(&current) {
        nodes.iter().for_each(|next| {
            // if !visited.contains(o) {
            queue.push_back((current, *next));
            // }
        });
    }
}

fn draw_map(map: &Vec<u32>, rows: usize, cols: usize, start: usize, target: usize) {
    let max = 340; // ((rows * cols - 1) / 10) as u32;
    let t = format!("{}", 178 as char);
    let a = '0' as u8;
    for r in 0..rows {
        (0..cols).for_each(|col| {
            let idx = r * cols + col;
            let (color, txt) = if idx == start {
                ([255, 255, 0], "S")
            } else if idx == target {
                ([0, 255, 255], "E")
            } else {
                let v = map[idx];
                (
                    if v > max {
                        [0, 0, 0]
                    } else {
                        let factor = v as f32 / max as f32;
                        interpolate_color([255, 255, 255], [255, 0, 0], factor)
                    },
                    "X",
                )
            };
            print!("{}", txt.truecolor(color[0], color[1], color[2]));
        });
        println!("");
    }
    println!("---");
}

fn quiz1() -> u32 {
    let input = read_file(FILE_NAME);
    proc1(&input)
}

fn build_graph2(grid: &Grid) -> HashMap<usize, Vec<usize>> {
    let map = &grid.grid;
    map.iter()
        .enumerate()
        .map(|(idx, center)| {
            let center = *center as i16;
            let row = idx / grid.cols;
            let col = idx % grid.cols;

            let mut n_idx = vec![];
            if row > 0 {
                n_idx.push((row - 1) * grid.cols + col);
            }
            if row < grid.rows - 1 {
                n_idx.push((row + 1) * grid.cols + col);
            }
            if col > 0 {
                n_idx.push(row * grid.cols + col - 1);
            }
            if col < grid.cols - 1 {
                n_idx.push(row * grid.cols + col + 1);
            }

            (
                idx,
                n_idx
                    .into_iter()
                    .filter(|&idx| center - map[idx] as i16 <= 1)
                    .collect::<_>(),
            )
        })
        .collect()
}
fn proc2(input: &str) -> u32 {
    let grid = load(input);
    println!("{grid:?}");

    let graph = build_graph2(&grid);
    println!("{graph:?}");

    let mut dist = vec![u32::MAX; grid.rows * grid.cols];
    dist[grid.target] = 0;

    let mut visited = HashSet::new();
    visited.insert(grid.target);

    let mut next_pos = VecDeque::new();
    push_back_next_nodes(&graph, grid.target, &visited, &mut next_pos);

    println!("{:?}", next_pos);
    while let Some((from, to)) = next_pos.pop_front() {
        println!("{from} -> {to}");

        let next = dist[from] + 1;
        if dist[to] > next {
            dist[to] = next;
        }

        if visited.contains(&to) {
            continue;
        }

        visited.insert(to);
        push_back_next_nodes(&graph, to, &visited, &mut next_pos);
        draw_map(&dist, grid.rows, grid.cols, grid.start, grid.target);
    }

    let mut min_dist = u32::MAX;
    for (idx, v) in grid.grid.iter().enumerate() {
        if grid.grid[idx] == 0 {
            if dist[idx] < min_dist {
                min_dist = dist[idx];
            }
        }
    }

    min_dist
}

fn quiz2() -> u32 {
    let input = read_file(FILE_NAME);
    proc2(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    #[ignore]
    fn test_load() {
        let grid = load(INPUT);
        assert_eq!(
            grid,
            Grid {
                grid: vec![
                    0, 0, 1, 16, 15, 14, 13, 12, 0, 1, 2, 17, 24, 23, 23, 11, 0, 2, 2, 18, 25, 25,
                    23, 10, 0, 2, 2, 19, 20, 21, 22, 9, 0, 1, 3, 4, 5, 6, 7, 8
                ],
                rows: 5,
                cols: 8,
                start: 0,
                target: 21
            }
        );
    }

    #[test]
    #[ignore]
    fn test_proc1() {
        assert_eq!(proc1(INPUT), 31);
    }

    #[test]
    #[ignore]
    fn test_quiz1() {
        assert_eq!(quiz1(), 339);
    }

    #[test]
    #[ignore]
    fn test_proc2() {
        assert_eq!(proc2(INPUT), 29);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(), 332);
    }
}
