fn tick_one(current: i32, neighbors: i32) -> i32 {
    if current == 1 {
        if neighbors == 2 || neighbors == 3 {
            1
        } else {
            0
        }
    } else {
        if neighbors == 3 {
            1
        } else {
            0
        }
    }
}

fn count_neighbors((row, col): (i32, i32), (rows, cols): (i32, i32), grid: &Vec<i32>) -> i32 {
    [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .iter()
    .map(|(r, c)| {
        if *r >= 0 && *r < rows && *c >= 0 && *c < cols && grid[(r * cols + c) as usize] == 1 {
            1
        } else {
            0
        }
    })
    .sum()
}

fn build_grid(text: &str) -> (i32, i32, Vec<i32>) {
    let rows = text.lines().count();
    let cols = text.lines().next().unwrap().chars().count();
    let grid: Vec<i32> = text
        .lines()
        .flat_map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }))
        .collect();

    (rows as i32, cols as i32, grid)
}

fn tick_grid(size: (i32, i32), grid: Vec<i32>) -> Vec<i32> {
    let (rows, cols) = size;
    (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .map(|coord| {
            let count = count_neighbors(coord, size, &grid);
            let (r, c) = coord;
            tick_one(grid[(r * cols + c) as usize], count)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn quiz1() {
        let text = read_file("../data/2015/input18.txt");
        let (rows, cols, mut grid) = build_grid(&text);

        for _ in 0..100 {
            grid = tick_grid((rows, cols), grid);
        }

        assert_eq!(grid.iter().sum::<i32>(), 821);
    }

    #[test]
    fn quiz2() {
        let text = read_file("../data/2015/input18.txt");
        let (rows, cols, mut grid) = build_grid(&text);

        for _ in 0..100 {
            grid[0] = 1;
            grid[(rows - 1) as usize] = 1;
            grid[((rows - 1) * cols) as usize] = 1;
            grid[(rows * cols - 1) as usize] = 1;
            grid = tick_grid((rows, cols), grid);
        }

        grid[0] = 1;
        grid[(rows - 1) as usize] = 1;
        grid[((rows - 1) * cols) as usize] = 1;
        grid[(rows * cols - 1) as usize] = 1;
        assert_eq!(grid.iter().sum::<i32>(), 886);
    }

    #[test]
    fn test_build_grid() {
        let (rows, cols, grid) = build_grid(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );
        assert_eq!((rows, cols), (6, 6));
        assert_eq!(grid[0], 0);
        assert_eq!(grid[1], 1);
        assert_eq!(grid[33], 1);
    }

    #[test]
    fn test_count_neighbors() {
        let (rows, cols, grid) = build_grid(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );

        assert_eq!(count_neighbors((0, 0), (rows, cols), &grid), 1);
        assert_eq!(count_neighbors((0, 2), (rows, cols), &grid), 3);
        assert_eq!(count_neighbors((4, 1), (rows, cols), &grid), 6);
    }

    #[test]
    fn test_tick_grid() {
        let (_rows, _cols, grid_1) = build_grid(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );
        let (rows, cols, grid_2) = build_grid(
            r"..##..
..##.#
...##.
......
#.....
#.##..",
        );

        assert_eq!(tick_grid((rows, cols), grid_1), grid_2);
    }
}
