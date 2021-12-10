fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn is_match_close(open: char, close: char) -> bool {
    (open == '(' && close == ')')
        || (open == '[' && close == ']')
        || (open == '{' && close == '}')
        || (open == '<' && close == '>')
}

fn matching_close(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn close_point(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn is_corrupted_or_remaining(line: &str) -> Result<Vec<char>, i64> {
    let cs = line.chars().collect::<Vec<char>>();
    let mut rs = vec![cs[0]];

    for i in 1..cs.len() {
        let c = cs[i];
        if is_open(c) {
            rs.push(c);
        } else {
            let left = rs[rs.len() - 1];
            if is_match_close(left, c) {
                rs.pop();
            } else {
                return Result::Err(close_point(c));
            }
        }
    }

    Result::Ok(rs)
}

fn complete_point(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

pub fn auto_complete_score(cs: &Vec<char>) -> i64 {
    let mut score = 0;
    for c in cs.iter() {
        score *= 5;
        score += complete_point(*c);
    }
    score
}

fn remaining_close(mut os: Vec<char>) -> Vec<char> {
    let mut cs = vec![];
    while let Some(c) = os.pop() {
        cs.push(matching_close(c));
    }
    cs
}

pub fn quiz2(text: &str) -> i64 {
    let mut scores = text
        .lines()
        .map(|line| is_corrupted_or_remaining(line))
        .filter_map(|x| match x {
            Ok(rs) => {
                let cs = remaining_close(rs);
                let s = auto_complete_score(&cs);
                Some(s)
            }
            Err(_) => None,
        })
        .collect::<Vec<i64>>();

    scores.sort();
    scores[scores.len() / 2]
}

pub fn quiz1(text: &str) -> i64 {
    text.lines()
        .map(|line| is_corrupted_or_remaining(line))
        .filter_map(|x| match x {
            Ok(_) => None,
            Err(x) => Some(x),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[allow(unused)]
    const INPUT: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_auto_complete_score() {
        assert_eq!(
            auto_complete_score(&("}}]])})]".chars().collect::<Vec<char>>())),
            288957
        );
    }

    #[test]
    fn test_is_corrupted() {
        assert!(is_corrupted_or_remaining("(]").is_err());
        assert!(is_corrupted_or_remaining("()").is_ok());
        assert!(is_corrupted_or_remaining("{([(<{}[<>[]}>{[]{[(<()>").is_err());
    }

    #[test]
    fn run_2021_d10_quiz1() {
        let text = read_file("data/2021/input10.txt");
        assert_eq!(quiz1(text.as_str().trim()), 390993);
    }

    #[test]
    fn run_2021_d10_quiz2() {
        let text = read_file("data/2021/input10.txt");
        assert_eq!(quiz2(text.as_str().trim()), 2391385187);
    }
}
