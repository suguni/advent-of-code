use regex::Regex;

struct Ingredient(i32, i32, i32, i32, i32);

fn parse_line(line: &str) -> Ingredient {

    let re = Regex::new(
        r"\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)")
        .unwrap();

    let caps = re.captures(line).unwrap();

    Ingredient(
        caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
    )
}

fn load_ingredients(text: &str) -> Vec<Ingredient> {
    text.lines()
        .map(|line| parse_line(line))
        .collect()
}

// fn combinations(value: i32, slots: usize) -> usize {
//     if slots == 1 {
//         1
//     } else {
//         for i in 0..=value {
//             combinations(i, slots - 1);
//         }
//
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_line("Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5"),
                   (4, -2, 0, 0, 5));
    }
}
