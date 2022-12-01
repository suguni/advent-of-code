#![allow(dead_code)]
use crate::read_file;

fn quiz1() -> i32 {
    let text = read_file("data/2022/input1.txt");
    most_calories(text)
}

fn quiz2() -> i32 {
    let text = read_file("data/2022/input1.txt");
    top_three_calories(text)
}

fn most_calories(data: String) -> i32 {
    let calories = load(data);
    *calories.iter().max().unwrap()
}

fn top_three_calories(data: String) -> i32 {
    let mut calories = load(data);
    calories.sort_by(|a, b| b.cmp(a));
    calories[0] + calories[1] + calories[2]
}

fn load(data: String) -> Vec<i32> {
    let mut calories = Vec::new();
    let mut s = 0;

    for line in data.lines() {
        if line.is_empty() {
            calories.push(s);
            s = 0;
        } else {
            s += line.parse::<i32>().unwrap();
        }
    }

    if s != 0 {
        calories.push(s);
    }

    calories
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calories() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(most_calories(data.clone().to_owned()), 24000);
        assert_eq!(top_three_calories(data.to_owned()), 45000);
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 70509);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 208567);
    }
}
