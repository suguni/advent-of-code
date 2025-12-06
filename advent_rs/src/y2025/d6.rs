const QUIZ_INPUT: &str = include_str!("../../data/2025/input6.txt");

fn quiz1() -> u64 {
    solve1(QUIZ_INPUT)
}

fn quiz2() -> u64 {
    solve2(QUIZ_INPUT)
}

fn solve1(input: &str) -> u64 {
    let data = parse_data1(input);
    data.calc()
}

fn solve2(input: &str) -> u64 {
    let data = parse_data2(input);
    data.calc()
}

fn parse_data1(input: &str) -> Data {
    let lines = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    let row_nums = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.iter()
                .map(|&ns| ns.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut nums = vec![];
    let cols = row_nums.first().unwrap().len();
    for c in 0..cols {
        let mut group = vec![];
        for r in 0..row_nums.len() {
            group.push(row_nums[r][c]);
        }
        nums.push(group);
    }

    let ops = lines
        .last()
        .unwrap()
        .iter()
        .map(|&c| c.chars().next().unwrap())
        .collect::<Vec<_>>();

    Data { nums, ops }
}

struct Data {
    nums: Vec<Vec<u64>>,
    ops: Vec<char>,
}

impl Data {
    fn calc(&self) -> u64 {
        self.nums
            .iter()
            .enumerate()
            .map(|(i, ns)| {
                let op = self.ops[i];
                match op {
                    '*' => ns.iter().product::<u64>(),
                    '+' => ns.iter().sum::<u64>(),
                    _ => panic!("invalid operator"),
                }
            })
            .sum()
    }
}

fn parse_data2(input: &str) -> Data {
    let mut lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ops = lines
        .split_off(lines.len() - 1)
        .into_iter()
        .flatten()
        .filter(|&c| c != ' ')
        .rev()
        .collect::<Vec<_>>();

    let num_cs = lines;
    let rows = num_cs.len();
    let cols = num_cs.first().unwrap().len();

    let mut nums: Vec<Vec<u64>> = vec![vec![]];

    for c in (0..cols).rev() {
        let mut ns = vec![];
        for r in 0..rows {
            let d = num_cs[r][c];
            if d != ' ' {
                ns.push(d);
            }
        }

        if ns.is_empty() {
            nums.push(vec![]);
        } else {
            let n = ns.iter().collect::<String>().parse::<u64>().unwrap();
            nums.last_mut().unwrap().push(n);
        }
    }

    Data { nums, ops }
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    use super::*;

    #[test]
    fn test_parse1() {
        let data = parse_data1(SAMPLE);
        assert_eq!(
            data.nums,
            vec![
                vec![123, 45, 6],
                vec![328, 64, 98],
                vec![51, 387, 215],
                vec![64, 23, 314],
            ]
        );
        assert_eq!(data.ops, vec!['*', '+', '*', '+']);
    }

    #[test]
    fn test_parse2() {
        let data = parse_data2(SAMPLE);
        assert_eq!(
            data.nums,
            vec![
                vec![4, 431, 623],
                vec![175, 581, 32],
                vec![8, 248, 369],
                vec![356, 24, 1],
            ]
        );
        assert_eq!(data.ops, vec!['+', '*', '+', '*']);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(SAMPLE), 4277556);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 4583860641327);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(SAMPLE), 3263827);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 11602774058280);
    }
}
