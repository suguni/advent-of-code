use std::collections::HashMap;

fn load_data(text: &str) -> (Vec<char>, HashMap<Vec<char>, char>) {
    let mut insertion_rules = HashMap::new();
    let mut lines = text.lines();

    let template = lines.next().unwrap().chars().collect();

    lines.next();

    for line in lines {
        let (l, r) = line.split_once(" -> ").unwrap();

        let chars = l.chars().collect::<Vec<char>>();
        insertion_rules.insert(chars, r.chars().next().unwrap());
    }

    (template, insertion_rules)
}

fn step(template: &Vec<char>, rules: &HashMap<Vec<char>, char>) -> Vec<char> {
    let last = template[template.len() - 1];
    let mut result = template
        .windows(2)
        .flat_map(|cs| {
            let v = rules.get(cs).unwrap();
            vec![cs[0], *v]
        })
        .collect::<Vec<char>>();

    result.push(last);
    result
}

fn count_elem(text: &Vec<char>) -> (usize, usize) {
    let m = text
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, c| {
            *acc.entry(*c).or_default() += 1;
            acc
        });

    let max = m.values().max().unwrap();
    let min = m.values().min().unwrap();

    (*min, *max)
}

pub fn quiz1(text: &str, count: usize) -> usize {
    let (mut template, rules) = load_data(text);
    (0..count).for_each(|_| {
        template = step(&template, &rules);
    });

    let (min, max) = count_elem(&template);
    max - min
}

fn step2(
    template: &HashMap<Vec<char>, usize>,
    rule: &HashMap<Vec<char>, char>,
) -> HashMap<Vec<char>, usize> {
    let mut new = HashMap::<Vec<char>, usize>::new();

    for (k, v) in template {
        let mid = rule.get(k).unwrap();
        *new.entry(vec![k[0], *mid]).or_default() += v;
        *new.entry(vec![*mid, k[1]]).or_default() += v;
    }

    new
}

fn count_elem2(template: &HashMap<Vec<char>, usize>, first: char, last: char) -> (usize, usize) {
    let mut result = HashMap::<char, usize>::new();

    for (k, v) in template {
        *result.entry(k[0]).or_default() += v;
        *result.entry(k[1]).or_default() += v;
    }

    *result.get_mut(&first).unwrap() += 1;
    *result.get_mut(&last).unwrap() += 1;

    let max = result.values().max().unwrap() / 2;
    let min = result.values().min().unwrap() / 2;

    (min, max)
}

pub fn quiz2(text: &str, count: usize) -> usize {
    let (template, rules) = load_data(text);

    let first = template[0];
    let last = template[template.len() - 1];

    let mut template =
        template
            .windows(2)
            .fold(HashMap::<Vec<char>, usize>::new(), |mut acc, cs| {
                *acc.entry(cs.to_vec()).or_default() += 1;
                acc
            });

    (0..count).for_each(|_| {
        template = step2(&template, &rules);
    });

    let (min, max) = count_elem2(&template, first, last);
    max - min
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const DATA1: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_load_data() {
        let (template, rules) = load_data(DATA1.trim());
        assert_eq!(template, "NNCB".chars().collect::<Vec<char>>());
        assert_eq!(rules.len(), 16);
    }

    #[test]
    fn test_step() {
        let mut rules = HashMap::new();
        rules.insert(vec!['N', 'N'], 'C');
        rules.insert(vec!['N', 'C'], 'B');
        rules.insert(vec!['C', 'B'], 'H');
        let s = step(&vec!['N', 'N', 'C', 'B'], &rules);
        assert_eq!(s, vec!['N', 'C', 'N', 'B', 'C', 'H', 'B']);
    }

    #[test]
    fn test_count_elem() {
        let (min, max) = count_elem(&vec!['N', 'C', 'N', 'B', 'C', 'H', 'B']);
        assert_eq!(max, 2);
        assert_eq!(min, 1);
    }

    #[test]
    fn run_2021_d14_quiz1() {
        let c = quiz1(DATA1.trim(), 10);
        assert_eq!(c, 1588);

        let text = read_file("data/2021/input14.txt");
        assert_eq!(quiz1(text.as_str().trim(), 10), 2345);
    }

    #[test]
    fn run_2021_d14_quiz2() {
        let c = quiz2(DATA1.trim(), 40);
        assert_eq!(c, 2188189693529);

        let text = read_file("data/2021/input14.txt");
        assert_eq!(quiz2(text.as_str().trim(), 40), 2432786807053);
    }
}
