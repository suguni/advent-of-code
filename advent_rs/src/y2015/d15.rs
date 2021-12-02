use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
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
    text.lines().map(|line| parse_line(line)).collect()
}

fn calc_score(ingredients: &Vec<Ingredient>, weights: Vec<i32>) -> (i32, i32) {
    let mut sum = [0; 4];
    let mut cal = 0;
    for i in 0..ingredients.len() {
        sum[0] += ingredients[i].0 * weights[i];
        sum[1] += ingredients[i].1 * weights[i];
        sum[2] += ingredients[i].2 * weights[i];
        sum[3] += ingredients[i].3 * weights[i];
        cal += ingredients[i].4 * weights[i];
    }

    (
        sum[0].max(0) * sum[1].max(0) * sum[2].max(0) * sum[3].max(0),
        cal,
    )
}

fn list_of_weights() -> impl std::iter::Iterator<Item = (i32, i32, i32, i32)> {
    (0..=100)
        .flat_map(|i| {
            (0..=(100 - i)).flat_map(move |j| {
                (0..=(100 - i - j))
                    .flat_map(move |k| (0..=(100 - i - j - k)).map(move |l| (i, j, k, l)))
            })
        })
        .filter(|(i, j, k, l)| i + j + k + l == 100)
}

fn max_score(ingredients: &Vec<Ingredient>) -> i32 {
    list_of_weights()
        .map(|(i, j, k, l)| {
            let (c, _cal) = calc_score(ingredients, vec![i, j, k, l]);
            c
        })
        .max()
        .unwrap()
}

fn max_score2(ingredients: &Vec<Ingredient>) -> i32 {
    list_of_weights()
        .filter_map(|(i, j, k, l)| {
            let (c, cal) = calc_score(ingredients, vec![i, j, k, l]);
            if cal == 500 {
                Some(c)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5"),
            Ingredient(4, -2, 0, 0, 5)
        );
    }

    #[test]
    fn quiz1() {
        let ingredients = load_ingredients(&read_file("../data/2015/input15.txt"));
        assert_eq!(max_score(&ingredients), 18965440);
    }

    #[test]
    fn quiz2() {
        let ingredients = load_ingredients(&read_file("../data/2015/input15.txt"));
        assert_eq!(max_score2(&ingredients), 15862900);
    }

    #[test]
    fn test_flat_map() {
        assert_eq!(
            (0..2)
                .flat_map(|i| (0..2).map(move |j| (i, j)))
                .collect::<Vec<(i32, i32)>>(),
            vec![(0, 0), (0, 1), (1, 0), (1, 1)]
        );
    }
}
