use super::*;

use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

use Substance::*;

#[derive(Hash, Eq, PartialEq)]
enum Substance {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl FromStr for Substance {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Substance::*;
        match s {
            "children" => Ok(Children),
            "cats" => Ok(Cats),
            "samoyeds" => Ok(Samoyeds),
            "pomeranians" => Ok(Pomeranians),
            "akitas" => Ok(Akitas),
            "vizslas" => Ok(Vizslas),
            "goldfish" => Ok(Goldfish),
            "trees" => Ok(Trees),
            "cars" => Ok(Cars),
            "perfumes" => Ok(Perfumes),
            _ => Err(())
        }
    }
}

struct Sue {
    no: i32,
    items: HashMap<Substance, i32>,
}

fn parse_line(line: &str) -> Sue {
    let re = Regex::new(
        r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)")
        .unwrap();

    let caps = re.captures(line).unwrap();

    let no = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let mut items = HashMap::new();
    for i in 0..3 {
        let key = caps.get(i * 2 + 2).unwrap().as_str().parse::<Substance>().unwrap();
        let value = caps.get(i * 2 + 3).unwrap().as_str().parse::<i32>().unwrap();
        items.insert(key, value);
    }

    Sue { no, items }
}


fn query_find<'a, T: Fn(&Sue, &HashMap<Substance, i32>) -> bool>(sues: &'a Vec<Sue>,
                  menu: &HashMap<Substance, i32>,
                  matcher: T) -> &'a Sue {
    sues.iter()
        .find(|sue| matcher(sue, menu))
        .unwrap()
}

fn match_mfcsam_1(sue: &Sue, menu: &HashMap<Substance, i32>) -> bool {
    sue.items
        .iter()
        .all(|(k, v)| menu[k] == *v)
}

fn match_mfcsam_2(sue: &Sue, menu: &HashMap<Substance, i32>) -> bool {
    sue.items
        .iter()
        .all(|(k, v)|
            match k {
                Children => *v == menu[&Children],
                Cats => *v > menu[&Cats],
                Samoyeds => *v == menu[&Samoyeds],
                Pomeranians => *v < menu[&Pomeranians],
                Akitas => *v == menu[&Akitas],
                Vizslas => *v == menu[&Vizslas],
                Goldfish => *v < menu[&Goldfish],
                Trees => *v > menu[&Trees],
                Cars => *v == menu[&Cars],
                Perfumes => *v == menu[&Perfumes],
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_menu() -> HashMap<Substance, i32> {
        let mut menu = HashMap::new();
        menu.insert(Children, 3);
        menu.insert(Cats, 7);
        menu.insert(Samoyeds, 2);
        menu.insert(Pomeranians, 3);
        menu.insert(Akitas, 0);
        menu.insert(Vizslas, 0);
        menu.insert(Goldfish, 5);
        menu.insert(Trees, 3);
        menu.insert(Cars, 2);
        menu.insert(Perfumes, 1);
        menu
    }

    #[test]
    fn quiz1() {
        let menu = create_menu();
        let sues: Vec<Sue> = read_file("../data/2015/input16.txt")
            .lines()
            .map(parse_line)
            .collect();

        let result = query_find(&sues, &menu, match_mfcsam_1);
        assert_eq!(result.no, 40);
    }

    #[test]
    fn quiz2() {
        let menu = create_menu();
        let sues: Vec<Sue> = read_file("../data/2015/input16.txt")
            .lines()
            .map(parse_line)
            .collect();

        let result = query_find(&sues, &menu, match_mfcsam_2);
        assert_eq!(result.no, 241);
    }

    #[test]
    fn test_parse_line() {
        let line = "Sue 168: pomeranians: 8, goldfish: 9, trees: 9";
        let sue = parse_line(line);

        assert_eq!(sue.no, 168);
        assert_eq!(sue.items[&Substance::Pomeranians], 8);
        assert_eq!(sue.items[&Substance::Goldfish], 9);
        assert_eq!(sue.items[&Substance::Trees], 9);
        assert!(!sue.items.contains_key(&Substance::Akitas));
    }
}