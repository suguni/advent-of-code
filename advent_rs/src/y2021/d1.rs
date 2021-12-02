use crate::read_file;

pub fn quiz1() -> usize {
    let nums = load("data/2021/input1.txt");
    count_increased(nums)
}

pub fn quiz2() -> usize {
    let nums = load("data/2021/input1.txt");
    let sums = moving_windows(nums, 3);
    count_increased(sums)
}

fn load(filename: &str) -> Vec<i32> {
    let text = read_file(filename);
    text.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

fn moving_windows(nums: Vec<i32>, size: usize) -> Vec<i32> {
    nums.windows(size)
        .map(|ns| ns.iter().sum())
        .collect::<Vec<i32>>()
}

fn count_increased(nums: Vec<i32>) -> usize {
    nums.windows(2)
        .map(|ps| ps[0] < ps[1])
        .filter(|&b| b)
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increased(nums), 7);
    }

    #[test]
    fn test_moving_windows() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(
            moving_windows(nums, 3),
            [607, 618, 618, 617, 647, 716, 769, 792]
        );
    }

    // #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 0);
    }

    // #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 0);
    }
}
