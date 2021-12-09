use std::collections::HashMap;

fn load_map(text: &str) -> (Vec<i8>, usize) {
    let cols = text.lines().next().unwrap().chars().count();
    let nums = text
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<i8>>();

    (nums, cols)
}

fn low_points(nums: &Vec<i8>, rows: usize, cols: usize) -> Vec<i8> {
    let mut points = vec![];

    let v = nums[0];
    let right = nums[1];
    let bottom = nums[cols];
    if v < right && v < bottom {
        points.push(v);
    }

    let v = nums[cols - 1];
    let left = nums[cols - 2];
    let bottom = nums[cols * 2 - 1];
    if v < left && v < bottom {
        points.push(v);
    }

    let v = nums[rows * (cols - 1)];
    let top = nums[rows * (cols - 2)];
    let right = nums[rows * (cols - 1) + 1];
    if v < top && v < right {
        points.push(v);
    }

    let v = nums[rows * cols - 1];
    let top = nums[(rows - 1) * cols];
    let left = nums[rows * cols - 2];
    if v < top && v < left {
        points.push(v);
    }

    for c in 1..(cols - 1) {
        // top line
        let v = nums[c];
        let left = nums[c - 1];
        let right = nums[c + 1];
        let bottom = nums[cols + c];

        if v < left && v < right && v < bottom {
            points.push(v);
        }

        // bottom line
        let base = (rows - 1) * cols;
        let v = nums[base + c];
        let left = nums[base + c - 1];
        let right = nums[base + c + 1];
        let top = nums[base - cols + c];

        if v < left && v < right && v < top {
            points.push(v);
        }
    }

    for r in 1..(rows - 1) {
        // left line
        let base = r * cols;
        let v = nums[base];
        let top = nums[base - cols];
        let bottom = nums[base + cols];
        let right = nums[base + 1];

        if v < top && v < bottom && v < right {
            points.push(v);
        }

        // right line
        let base = (r + 1) * cols - 1;
        let v = nums[base];
        let top = nums[base - cols];
        let bottom = nums[base + cols];
        let left = nums[base - 1];

        if v < top && v < bottom && v < left {
            points.push(v);
        }
    }

    for r in 1..(rows - 1) {
        for c in 1..(cols - 1) {
            let base = r * cols + c;
            let v = nums[base];
            let top = nums[base - cols];
            let right = nums[base + 1];
            let bottom = nums[base + cols];
            let left = nums[base - 1];

            if v < top && v < right && v < bottom && v < left {
                points.push(v);
            }
        }
    }

    points
}

pub fn quiz1(text: &str) -> i64 {
    let (nums, cols) = load_map(text);
    let pts = low_points(&nums, nums.len() / cols, cols);
    pts.iter().map(|v| *v as i64 + 1).sum::<i64>()
}

fn get_unvisited_neighbors(rows: usize, cols: usize, p: usize, visited: &Vec<bool>) -> Vec<usize> {
    let col = p % cols;
    let row = p / cols;

    let mut neighbors = vec![];

    if col != 0 && !visited[p - 1] {
        neighbors.push(p - 1);
    }

    if col != cols - 1 && !visited[p + 1] {
        neighbors.push(p + 1);
    }

    if row != 0 && !visited[p - cols] {
        neighbors.push(p - cols);
    }

    if row != rows - 1 && !visited[p + cols] {
        neighbors.push(p + cols);
    }

    neighbors
}

fn get_unvisited(visited: &Vec<bool>) -> Option<usize> {
    visited.iter().position(|p| !*p)
}

fn find_basins(map: &Vec<i8>, rows: usize, cols: usize) -> HashMap<usize, Vec<usize>> {
    let mut visited = vec![false; map.len()];

    let mut basin_id = 0;
    let mut basins: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut neighbors: Vec<usize> = vec![0];
    visited[0] = true;

    loop {
        while let Some(pos) = neighbors.pop() {
            if map[pos] != 9 {
                let pts = basins.entry(basin_id).or_insert(vec![]);
                pts.push(pos);

                let ns = get_unvisited_neighbors(rows, cols, pos, &visited);

                for n in ns.iter() {
                    visited[*n] = true;
                    neighbors.push(*n);
                }
            }
        }

        basin_id += 1;

        if let Some(p) = get_unvisited(&visited) {
            neighbors.push(p);
            visited[p] = true;
        } else {
            break;
        }
    }

    basins
}

pub fn quiz2(text: &str) -> u32 {
    let (map, cols) = load_map(text);
    let rows = map.len() / cols;

    let basins = find_basins(&map, rows, cols);

    let mut areas = basins
        .values()
        .map(|ps| ps.len() as u32)
        .collect::<Vec<u32>>();
    areas.sort();
    let len = areas.len();

    areas[len - 1] * areas[len - 2] * areas[len - 3]
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const MAP: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678
";

    fn counts(starts: &Vec<i8>) -> [i64; 9] {
        let mut counts = [0; 9];
        for s in starts {
            counts[*s as usize] += 1;
        }
        counts
    }

    #[test]
    fn test_load_map() {
        let (nums, cols) = load_map(MAP.trim());
        assert_eq!(cols, 10);
        assert_eq!(nums[..10], vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_low_points() {
        let (nums, cols) = load_map(MAP.trim());
        let pts = low_points(&nums, nums.len() / cols, cols);
        assert_eq!(counts(&pts), counts(&vec![1, 0, 5, 5]));
    }

    #[test]
    fn run_y2021_d9_quiz1() {
        let text = read_file("data/2021/input9.txt");
        assert_eq!(quiz1(text.as_str().trim()), 537);
    }

    #[test]
    fn test_find_basins() {
        let (map, cols) = load_map(MAP.trim());
        let rows = map.len() / cols;

        let basins = find_basins(&map, rows, cols);

        assert_eq!(basins.iter().count(), 4);
    }

    #[test]
    fn run_y2021_d9_quiz2() {
        let text = read_file("data/2021/input9.txt");
        assert_eq!(quiz2(text.as_str().trim()), 1142757);
    }
}
