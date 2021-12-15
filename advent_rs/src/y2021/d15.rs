fn load_data(text: &str) -> (Vec<i32>, usize, usize) {
    let rows = text.lines().count();
    let cols = text.lines().next().unwrap().chars().count();
    let levels = text
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<i32>>();

    (levels, rows, cols)
}

fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<usize> {
    vec![(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .filter_map(|(dr, dc)| {
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                Some((r * cols as i32 + c) as usize)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>()
}

fn gen_dist_map(levels: Vec<i32>, rows: usize, cols: usize) -> Vec<i32> {
    let mut dist = vec![i32::MAX; rows * cols];
    dist[0] = levels[0];

    let mut changed = false;

    loop {
        for p in 1..(rows * cols) {
            let row = p / cols;
            let col = p % cols;

            let mv = neighbors(row, col, rows, cols)
                .iter()
                .map(|p| dist[*p])
                .min()
                .unwrap();

            let d = mv + levels[p];

            if d != dist[p] {
                changed = true;
                dist[p] = d;
            }
        }
        if !changed {
            break;
        }

        changed = false;
    }

    dist
}

pub fn quiz1(text: &str) -> i32 {
    let (levels, rows, cols) = load_data(text);
    let start = levels[0];
    let dist = gen_dist_map(levels, rows, cols);
    dist[rows * cols - 1] - start
}

fn gen_full_map(levels: Vec<i32>, rows: usize, cols: usize) -> (Vec<i32>, usize, usize) {
    let mut full_map = vec![0; rows * cols * 25];
    let full_cols = cols * 5;
    let full_rows = cols * 5;

    for tr in 0..5 {
        for tc in 0..5 {
            let a = tr + tc;
            for i in 0..rows * cols {
                let nr = tr * rows + i / cols;
                let nc = tc * cols + i % cols;
                full_map[nr * full_cols + nc] = (levels[i] - 1 + a as i32) % 9 + 1;
            }
        }
    }

    (full_map, full_rows, full_cols)
}

pub fn quiz2(text: &str) -> i32 {
    let (levels, rows, cols) = load_data(text);
    let start = levels[0];
    let (full_map, rows, cols) = gen_full_map(levels, rows, cols);
    let dist = gen_dist_map(full_map, rows, cols);
    dist[rows * cols - 1] - start
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const DATA1: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_load_data() {
        let (levels, row, col) = load_data(DATA1.trim());
        assert_eq!(row, 10);
        assert_eq!(col, 10);
        assert_eq!(levels[0], 1);
        assert_eq!(levels[99], 1);
    }

    // 1 3
    // 2 1

    // 1 4
    // 3 4

    // 1 3 8
    // 2 1 3
    // 3 6 9

    // 1 4 12
    // 3 4 7
    // 6 10 16

    #[test]
    fn test_generate_dist_map() {
        assert_eq!(gen_dist_map(vec![1, 3, 2, 1], 2, 2), vec![1, 4, 3, 4]);
        assert_eq!(
            gen_dist_map(vec![1, 3, 8, 2, 1, 3, 3, 6, 9], 3, 3),
            vec![1, 4, 12, 3, 4, 7, 6, 10, 16]
        );
    }

    #[test]
    fn run_2021_d15_quiz1() {
        assert_eq!(quiz1(DATA1.trim()), 40);

        let text = read_file("data/2021/input15.txt");
        assert_eq!(quiz1(text.as_str().trim()), 714);
    }

    #[test]
    fn test_gen_full_map() {
        let (levels, rows, cols) = load_data(DATA1.trim());
        let (full_map, _, _) = gen_full_map(levels, rows, cols);
        assert_eq!(
            full_map[..50]
                .iter()
                .map(|i| i.to_string())
                .collect::<String>(),
            "11637517422274862853338597396444961841755517295286"
        );
    }

    #[test]
    fn run_2021_d15_quiz2() {
        assert_eq!(quiz2(DATA1.trim()), 315);

        let text = read_file("data/2021/input15.txt");
        assert_eq!(quiz2(text.as_str().trim()), 2948);
    }
}
