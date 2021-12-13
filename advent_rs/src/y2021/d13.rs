use regex::Regex;
use std::collections::HashSet;

enum Folding {
    X(i32),
    Y(i32),
}

fn load_data(text: &str) -> (HashSet<(i32, i32)>, i32, i32, Vec<Folding>) {
    let mut coords = HashSet::new();
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    let lines = text.lines().take_while(|line| *line != "");
    for line in lines {
        let (l, r) = line.split_once(',').unwrap();
        let x = l.parse::<i32>().unwrap();
        let y = r.parse::<i32>().unwrap();
        coords.insert((x, y));

        if x > width {
            width = x;
        }

        if y > height {
            height = y;
        }
    }

    let mut foldings = vec![];

    let re = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    let mut lines = text.lines().skip_while(|line| *line != "");
    lines.next();
    for line in lines {
        if let Some(cap) = re.captures_iter(line).next() {
            let folding = match &cap[1] {
                "x" => Folding::X(cap[2].parse::<i32>().unwrap()),
                "y" => Folding::Y(cap[2].parse::<i32>().unwrap()),
                _ => panic!(),
            };
            foldings.push(folding);
        }
    }

    (coords, width + 1, height + 1, foldings)
}

fn fold(
    coords: &HashSet<(i32, i32)>,
    width: i32,
    height: i32,
    folding: &Folding,
) -> (HashSet<(i32, i32)>, i32, i32) {
    let mut folded = HashSet::new();
    let mut new_width = width;
    let mut new_height = height;

    match folding {
        Folding::X(c) => {
            for (x, y) in coords.iter() {
                let x = if *x > *c { c - (*x - c) } else { *x };
                folded.insert((x, *y));
            }
            new_width = *c;
        }
        Folding::Y(c) => {
            for (x, y) in coords {
                let y = if *y > *c { c - (*y - c) } else { *y };
                folded.insert((*x, y));
            }
            new_height = *c;
        }
    }

    (folded, new_width, new_height)
}

pub fn quiz1(text: &str) -> usize {
    let (coords, width, height, foldings) = load_data(text.trim());
    let (coords, _, _) = fold(&coords, width, height, &foldings[0]);
    coords.len()
}

pub fn quiz2(text: &str) {
    let (mut coords, mut width, mut height, foldings) = load_data(text.trim());
    for folding in foldings.iter() {
        let n = fold(&coords, width, height, folding);
        coords = n.0;
        width = n.1;
        height = n.2;
    }

    print_paper(&coords, width, height);
}

fn print_paper(coords: &HashSet<(i32, i32)>, width: i32, height: i32) {
    let mut msg = String::new();
    for y in 0..height {
        for x in 0..width {
            if coords.contains(&(x, y)) {
                msg.push('#');
            } else {
                msg.push('.');
            }
        }
        msg.push('\n');
    }
    println!("{}", msg);
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const DATA1: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_load_data() {
        let (coords, width, height, foldings) = load_data(DATA1.trim());
        assert_eq!(coords.len(), 18);
        assert_eq!(width, 11);
        assert_eq!(height, 15);
        assert_eq!(foldings.len(), 2);
    }

    #[test]
    fn test_fold() {
        let (coords, width, height, foldings) = load_data(DATA1.trim());

        let (coords, width, height) = fold(&coords, width, height, &foldings[0]);
        assert_eq!(coords.len(), 17);
        assert_eq!(width, 11);
        assert_eq!(height, 7);

        let (coords, width, height) = fold(&coords, width, height, &foldings[1]);
        assert_eq!(coords.len(), 16);
        assert_eq!(width, 5);
        assert_eq!(height, 7);
    }

    #[test]
    fn run_2021_d13_quiz1() {
        let text = read_file("data/2021/input13.txt");
        assert_eq!(quiz1(text.as_str().trim()), 695);
    }

    #[test]
    fn run_2021_d13_quiz2() {
        let text = read_file("data/2021/input13.txt");
        quiz2(text.as_str().trim());
    }
}
