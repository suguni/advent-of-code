use crate::read_file;

pub fn quiz1() -> usize {
    let text = read_file("data/2021/input1.txt");
    let nums: Vec<i32> = text.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    count_increased(nums)
}

pub fn count_increased(nums: Vec<i32>) -> usize {
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
    fn run_quiz1() {
        assert_eq!(quiz1(), 0);
    }
}
