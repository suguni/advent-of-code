const QUIZ_INPUT: &str = include_str!("../../data/2025/input3.txt");

fn quiz1() -> u32 {
    solve1(QUIZ_INPUT)
}

fn solve1(data: &str) -> u32 {
    data.lines()
        .map(|line| largest_joltage(line))
        .sum()
}

fn solve2(data: &str) -> u64 {
    data.lines()
        .map(|line| largest_joltage2(line, 12))
        .sum()
}

fn quiz2() -> u64 {
    solve2(QUIZ_INPUT)
}

fn first_num_index(input: &str) -> usize {
    input.chars()
        .take(input.len() - 1)
        .enumerate()
        .max_by(|(i, a), (j, b)| a.cmp(b).then(j.cmp(i)))
        .unwrap()
        .0
}

fn second_num(input: &str, first_index: usize) -> u32 {
    input[first_index + 1..]
        .chars()
        .max()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn largest_joltage(input: &str) -> u32 {
    let i = first_num_index(input);
    let n = second_num(input, i);
    let m = input.chars()
        .nth(i)
        .unwrap()
        .to_digit(10)
        .unwrap();
    m * 10 + n
}

/*
input 의 start index 에서 시작해서 size 길이 숫자를 만들때 가장 큰 값과 위치를 반환
 */
fn largest_num_index(input: &str, start: usize, size: usize) -> (usize, u32) {
    input[start..=(input.len() - size)]
        .chars()
        .enumerate()
        .map(|(i, c)| (i + start, c.to_digit(10).unwrap()))
        .max_by(|(i, a), (j, b)| a.cmp(b).then(j.cmp(i)))
        .unwrap()
}

fn largest_joltage2(input: &str, size: usize) -> u64 {
    let mut start = 0;
    let mut result: u64 = 0;
    for i in (0..size).rev() {
        let (p, v) = largest_num_index(input, start, i + 1);
        start = p + 1;
        result = result * 10 + v as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    use super::*;

    #[test]
    fn test_first_num_index() {
        assert_eq!(first_num_index("987654321111111"), 0);
        assert_eq!(first_num_index("818181911112111"), 6);
        assert_eq!(first_num_index("818182111112111"), 0);
        assert_eq!(first_num_index("818182111112119"), 0);
    }

    #[test]
    fn test_second_num() {
        assert_eq!(second_num("987654321111111", 0), 8);
        assert_eq!(second_num("818181911112111", 6), 2);
        assert_eq!(second_num("818182111112111", 0), 8);
        assert_eq!(second_num("818182111112119", 0), 9);
    }

    #[test]
    fn test_largest_joltage() {
        assert_eq!(largest_joltage("987654321111111"), 98);
        assert_eq!(largest_joltage("811111111111119"), 89);
        assert_eq!(largest_joltage("818181911112111"), 92);
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 357);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 17332);
    }

    #[test]
    fn test_largest_num_index() {
        assert_eq!(largest_num_index("69879", 0, 4), (1, 9));

        assert_eq!(largest_num_index("89769879", 0, 6), (1, 9));
        assert_eq!(largest_num_index("89769879", 2, 5), (2, 7));
        assert_eq!(largest_num_index("89769879", 3, 4), (4, 9));
        assert_eq!(largest_num_index("89769879", 5, 3), (5, 8));
        assert_eq!(largest_num_index("89769879", 6, 2), (6, 7));
        assert_eq!(largest_num_index("89769879", 7, 1), (7, 9));
    }

    #[test]
    fn test_largets_joltage2() {
        assert_eq!(largest_joltage2("89769879", 6), 979879);

        assert_eq!(largest_joltage2("987654321111111", 12), 987654321111);
        assert_eq!(largest_joltage2("811111111111119", 12), 811111111119);
        assert_eq!(largest_joltage2("234234234234278", 12), 434234234278);
        assert_eq!(largest_joltage2("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 3121910778619);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 172516781546707);
    }
}
