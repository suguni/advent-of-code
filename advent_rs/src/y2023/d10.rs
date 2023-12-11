use std::collections::{HashMap, HashSet, LinkedList};
use std::iter::FromIterator;

const INPUT: &str = include_str!("../../data/2023/input10.txt");

type Position = (usize, usize);

struct Map {
    tiles: Vec<Vec<Tile>>,
    size: (usize, usize),
    start: Position,
}

impl Map {
    fn neighbors(&self, (cr, cc): Position) -> Vec<Position> {
        let mut result = vec![];

        let ct = self.tiles[cr][cc];

        if cc > 0 {
            if ct.can_connect_to_west(self.tiles[cr][cc - 1]) {
                result.push((cr, cc - 1));
            }
        }

        if cc < self.size.1 - 1 {
            if ct.can_connect_to_east(self.tiles[cr][cc + 1]) {
                result.push((cr, cc + 1));
            }
        }

        if cr > 0 {
            if ct.can_connect_to_north(self.tiles[cr - 1][cc]) {
                result.push((cr - 1, cc));
            }
        }

        if cr < self.size.0 - 1 {
            if ct.can_connect_to_south(self.tiles[cr + 1][cc]) {
                result.push((cr + 1, cc));
            }
        }

        result
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum Tile {
    VR,
    HO,
    NE,
    NW,
    SW,
    SE,
    ST,
    GR,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '|' => Self::VR,
            '-' => Self::HO,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::ST,
            _ => Self::GR,
        }
    }

    fn can_connect_to_north(&self, to: Tile) -> bool {
        (*self == Tile::ST || *self == Tile::VR || *self == Tile::NE || *self == Tile::NW)
            && (to == Tile::ST || to == Tile::VR || to == Tile::SE || to == Tile::SW)
    }

    fn can_connect_to_south(&self, to: Tile) -> bool {
        (*self == Tile::ST || *self == Tile::VR || *self == Tile::SE || *self == Tile::SW)
            && (to == Tile::ST || to == Tile::VR || to == Tile::NE || to == Tile::NW)
    }

    fn can_connect_to_east(&self, to: Tile) -> bool {
        (*self == Tile::ST || *self == Tile::HO || *self == Tile::SE || *self == Tile::NE)
            && (to == Tile::ST || to == Tile::HO || to == Tile::NW || to == Tile::SW)
    }

    fn can_connect_to_west(&self, to: Tile) -> bool {
        (*self == Tile::ST || *self == Tile::HO || *self == Tile::SW || *self == Tile::NW)
            && (to == Tile::ST || to == Tile::HO || to == Tile::NE || to == Tile::SE)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn load(data: &str) -> Map {
    let tiles = data
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile::new).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let start = tiles
        .iter()
        .map(|vs| vs.iter().position(|c| c == &Tile::ST))
        .enumerate()
        .find(|(_, c)| c.is_some())
        .map(|(r, c)| (r, c.unwrap()))
        .unwrap();

    let size = (tiles.len(), tiles[0].len());

    Map { tiles, start, size }
}

fn distance_map(map: &Map) -> HashMap<Position, usize> {
    let mut visited = HashMap::new();

    let mut neighbors_stack: Vec<(Position, Position, usize)> = vec![(map.start, map.start, 0)];

    while let Some((from_pos, curr_pos, curr_steps)) = neighbors_stack.pop() {
        let mut next = true;

        if let Some(steps) = visited.get_mut(&curr_pos) {
            if curr_pos == map.start {
                if *steps < curr_steps {
                    *steps = curr_steps;
                }
                next = false;
            }

            if *steps > curr_steps {
                next = false;
            }
        }

        if next {
            visited.insert(curr_pos, curr_steps);

            let neighbors = map.neighbors(curr_pos);

            for n_pos in neighbors {
                if n_pos != from_pos {
                    neighbors_stack.push((curr_pos, n_pos, curr_steps + 1));
                }
            }
        }
    }

    visited
}

fn solve1(data: &str) -> usize {
    let map = load(data);
    let visited = distance_map(&map);
    visited.get(&map.start).unwrap() / 2
}

fn boundary(map: &Map, visited: HashMap<Position, usize>) -> (HashSet<Position>, Vec<Position>) {
    let mut cleaned: HashSet<Position> = HashSet::new();
    let mut path: Vec<Position> = vec![];

    let mut position = map.start;
    loop {
        let steps = *visited.get(&position).unwrap();
        cleaned.insert(position);
        path.push(position);

        let mut found = false;

        for neighbor in map.neighbors(position) {
            if let Some(n_steps) = visited.get(&neighbor) {
                if steps - 1 == *n_steps {
                    position = neighbor;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            break;
        }
    }

    (cleaned, path)
}

fn enclosed(map: &Map, boundary: &HashSet<Position>, path: &Vec<Position>) -> HashSet<Position> {
    // let filled: Vec<u8> = vec![0; map.size.0 * map.size.1];

    let mut filled = HashSet::new();

    for i in 1..path.len() {
        let p = path[i - 1];

        // FIXME find ccw / cw
        let diffs = diffs_counter_clock(path[i - 1], map.tiles[p.0][p.1], path[i]);

        for diff in diffs {
            let mut r = p.0 as i32;
            let mut c = p.1 as i32;

            loop {
                r += diff.0;
                c += diff.1;

                if r <= 0
                    || r >= map.size.0 as i32 - 1
                    || c <= 0
                    || c >= map.size.1 as i32 - 1
                    || boundary.contains(&(r as usize, c as usize))
                {
                    break;
                } else {
                    filled.insert((r as usize, c as usize));
                }
            }
        }
    }

    filled
}

fn diffs_clock(
    (from_r, from_c): Position,
    from_tile: Tile,
    (to_r, to_c): Position,
) -> Vec<(i32, i32)> {
    if from_r == to_r {
        if from_c + 1 == to_c {
            // move east
            if from_tile == Tile::NE {
                vec![(1, 0), (0, -1)]
            } else if from_tile == Tile::HO {
                vec![(1, 0)]
            } else {
                vec![]
            }
        } else if from_c - 1 == to_c {
            // move west
            if from_tile == Tile::SW {
                vec![(-1, 0), (0, 1)]
            } else if from_tile == Tile::HO {
                vec![(-1, 0)]
            } else {
                vec![]
            }
        } else {
            panic!();
        }
    } else if from_c == to_c {
        if from_r + 1 == to_r {
            // move south
            if from_tile == Tile::SE {
                vec![(0, -1), (-1, 0)]
            } else if from_tile == Tile::VR {
                vec![(0, -1)]
            } else {
                vec![]
            }
        } else if from_r - 1 == to_r {
            // move north
            if from_tile == Tile::NW {
                vec![(0, 1), (1, 0)]
            } else if from_tile == Tile::VR {
                vec![(0, 1)]
            } else {
                vec![]
            }
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn diffs_counter_clock(
    (from_r, from_c): Position,
    from_tile: Tile,
    (to_r, to_c): Position,
) -> Vec<(i32, i32)> {
    if from_r == to_r {
        if from_c + 1 == to_c {
            // move east
            if from_tile == Tile::SE {
                vec![(-1, 0), (0, -1)]
            } else if from_tile == Tile::HO {
                vec![(-1, 0)]
            } else {
                vec![]
            }
        } else if from_c - 1 == to_c {
            // move west
            if from_tile == Tile::NW {
                vec![(1, 0), (0, 1)]
            } else if from_tile == Tile::HO {
                vec![(1, 0)]
            } else {
                vec![]
            }
        } else {
            panic!();
        }
    } else if from_c == to_c {
        if from_r + 1 == to_r {
            // move south
            if from_tile == Tile::SW {
                vec![(0, 1), (-1, 0)]
            } else if from_tile == Tile::VR {
                vec![(0, 1)]
            } else {
                vec![]
            }
        } else if from_r - 1 == to_r {
            // move north
            if from_tile == Tile::NE {
                vec![(0, -1), (1, 0)]
            } else if from_tile == Tile::VR {
                vec![(0, -1)]
            } else {
                vec![]
            }
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn solve2(data: &str) -> usize {
    let map = load(data);
    let visited = distance_map(&map);
    let (boundary, path) = boundary(&map, visited);
    let inner = enclosed(&map, &boundary, &path);
    inner.len()
}

fn print_boundary((rows, cols): (usize, usize), boundary: &HashSet<Position>) {
    let mut canvas = String::new();
    for r in 0..rows {
        for c in 0..cols {
            if boundary.contains(&(r, c)) {
                canvas.push('+');
            } else {
                canvas.push('.');
            }
        }
        canvas.push('\n');
    }
    println!("{canvas}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set;
    use crate::y2021::d2::Pos;

    const EXAMPLE1: &str = ".....
.F-7.
.|.|.
.L-J.
.....";

    const EXAMPLE2: &str = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EXAMPLE3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_boundary() {
        let map = load(EXAMPLE2);
        let visited = distance_map(&map);
        let (set, vec) = boundary(&map, visited);

        assert_eq!(
            set,
            set![
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 3),
                (3, 3),
                (3, 2),
                (3, 1),
                (2, 1)
            ]
        )
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE2), 4);
        assert_eq!(solve1(EXAMPLE3), 8);
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(solve1(INPUT), 6820);
    }

    #[test]
    fn test_load() {
        let Map { tiles, start, size } = load(EXAMPLE2);
        assert_eq!(start, (1, 1));
        assert_eq!(size, (5, 5));
    }

    const EXAMPLE4: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_inner() {
        let map = load(EXAMPLE4);
        let visited = distance_map(&map);
        let (boundary, path) = boundary(&map, visited);
        let inner = enclosed(&map, &boundary, &path);

        println!("{:?}", path);
        print_boundary(map.size, &boundary);

        assert_eq!(inner, set![(6, 2), (6, 3), (6, 7), (6, 8)])
    }

    #[test]
    fn test_inner2() {
        let map = load(EXAMPLE5);
        let visited = distance_map(&map);
        let (boundary, path) = boundary(&map, visited);
        let inner = enclosed(&map, &boundary, &path);

        println!("{:?}", path);
        print_boundary(map.size, &boundary);

        assert_eq!(
            inner,
            set![
                (3, 14),
                (4, 10),
                (4, 11),
                (4, 12),
                (4, 13),
                (5, 11),
                (5, 12),
                (5, 13),
                (6, 13),
                (6, 14)
            ]
        )
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE5), 10);
        assert_eq!(solve2(EXAMPLE4), 4);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(solve2(INPUT), 337);
    }
}
