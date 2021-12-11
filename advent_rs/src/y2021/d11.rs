fn load_text(text: &str) -> Vec<i32> {
    text.lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<i32>>()
}

fn neighbor_count_flashed(
    levels: &Vec<i32>,
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
) -> usize {
    let cols = cols as i32;
    let rows = rows as i32;
    let row = row as i32;
    let col = col as i32;

    let c = row * cols + col;
    let lt = c - cols - 1;
    let t = lt + 1;
    let rt = t + 1;
    let l = c - 1;
    let r = c + 1;
    let lb = c + cols - 1;
    let b = lb + 1;
    let rb = b + 1;

    let mut count = 0;

    if row != 0 {
        if levels[t as usize] > 9 {
            count += 1;
        }
    }

    if row != rows - 1 {
        if levels[b as usize] > 9 {
            count += 1;
        }
    }

    if col != 0 {
        if levels[l as usize] > 9 {
            count += 1;
        }
    }

    if col != cols - 1 {
        if levels[r as usize] > 9 {
            count += 1;
        }
    }

    if row != 0 && col != 0 {
        if levels[lt as usize] > 9 {
            count += 1;
        }
    }

    if row != 0 && col != cols - 1 {
        if levels[rt as usize] > 9 {
            count += 1;
        }
    }
    if row != rows - 1 && col != 0 {
        if levels[lb as usize] > 9 {
            count += 1;
        }
    }

    if row != rows - 1 && col != cols - 1 {
        if levels[rb as usize] > 9 {
            count += 1;
        }
    }

    count
}

fn step(levels: &mut Vec<i32>, rows: usize, cols: usize) {
    for v in levels.iter_mut() {
        *v += 1;
    }

    loop {
        let flashed_count = spread_flash(levels, rows, cols);

        if flashed_count == 0 {
            break;
        }
    }
}

fn spread_flash(levels: &mut Vec<i32>, rows: usize, cols: usize) -> usize {
    let len = levels.len();
    let mut flashed = vec![-1; len];
    let mut flashed_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            let p = row * cols + col;
            if levels[p] > 0 && levels[p] <= 9 {
                flashed[p] = neighbor_count_flashed(&levels, rows, cols, row, col) as i32;
                if flashed[p] > 0 {
                    flashed_count += 1;
                }
            }
        }
    }

    for i in 0..len {
        if flashed[i] == -1 {
            levels[i] = 0;
        } else {
            levels[i] += flashed[i];
        }
    }

    flashed_count
}

pub fn quiz1(text: &str, rows: usize, cols: usize) -> usize {
    let mut levels = load_text(text);
    let mut count = 0;
    for _ in 0..100 {
        step(&mut levels, rows, cols);
        count += levels.iter().filter(|v| **v == 0).count();
    }

    count
}

pub fn quiz2(text: &str, rows: usize, cols: usize) -> usize {
    let mut levels = load_text(text);

    for i in 0.. {
        step(&mut levels, rows, cols);
        if levels.iter().filter(|v| **v == 0).count() == rows * cols {
            return (i + 1) as usize;
        }
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const DATA: &str = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    const SMALL: &str = "
11111
19991
19191
19991
11111
";

    const SMALL_STEP1: &str = "
34543
40004
50005
40004
34543
";

    const SMALL_STEP2: &str = "
45654
51115
61116
51115
45654
";

    #[test]
    fn test_load_text() {
        let levels = load_text(DATA.trim());
        assert_eq!(levels.len(), 100);
        assert_eq!(levels[0], 5);
        assert_eq!(levels[99], 6);
    }

    #[test]
    fn test_neighbor_count_flashed() {
        let mut levels = load_text(SMALL.trim());
        for l in levels.iter_mut() {
            *l += 1;
        }

        assert_eq!(neighbor_count_flashed(&levels, 5, 5, 0, 0), 1);
        assert_eq!(neighbor_count_flashed(&levels, 5, 5, 0, 2), 3);
        assert_eq!(neighbor_count_flashed(&levels, 5, 5, 0, 4), 1);
        assert_eq!(neighbor_count_flashed(&levels, 5, 5, 1, 1), 2);
        assert_eq!(neighbor_count_flashed(&levels, 5, 5, 2, 2), 8);
    }

    // 1 2 3
    // 4 5 6
    // 7 8 9
    //
    // +1
    //
    // 2 3 4
    // 5 6 7
    // 8 9 0
    //
    // spread
    //
    // 2 3 4
    // 5 7 8
    // 8 0 _
    //
    // spread
    //
    // 2 3 4
    // 6 8 9
    // 9 _ _

    #[test]
    fn test_step() {
        let mut levels = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        step(&mut levels, 3, 3);
        assert_eq!(levels, vec![2, 3, 4, 6, 8, 9, 9, 0, 0]);

        let mut levels = load_text(SMALL.trim());
        step(&mut levels, 5, 5);
        assert_eq!(levels, load_text(SMALL_STEP1.trim()));
        step(&mut levels, 5, 5);
        assert_eq!(levels, load_text(SMALL_STEP2.trim()));
    }

    #[test]
    fn test_quiz1() {
        assert_eq!(quiz1(DATA.trim(), 10, 10), 1656);
    }

    #[test]
    fn test_quiz2() {
        assert_eq!(quiz2(DATA.trim(), 10, 10), 195);
    }

    #[test]
    fn run_2021_d11_quiz1() {
        let text = read_file("data/2021/input11.txt");
        assert_eq!(quiz1(text.as_str().trim(), 10, 10), 1627);
    }

    #[test]
    fn run_2021_d11_quiz2() {
        let text = read_file("data/2021/input11.txt");
        assert_eq!(quiz2(text.as_str().trim(), 10, 10), 329);
    }
}
