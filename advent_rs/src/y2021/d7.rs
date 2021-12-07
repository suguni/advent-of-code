pub fn total_fuel1(vs: &Vec<i32>, base: i32) -> i32 {
    vs.iter().map(|v| (base - *v).abs()).sum()
}

pub fn total_fuel2(vs: &Vec<i32>, base: i32) -> i32 {
    vs.iter()
        .map(|v| {
            let d = (base - *v).abs();
            d * (d + 1) / 2
        })
        .sum()
}

pub fn find_min_base(vs: &Vec<i32>, total_fuel: fn(&Vec<i32>, i32) -> i32) -> (i32, i32) {
    let (min_v, max_v) = vs
        .iter()
        .fold((i32::MAX, i32::MIN), |(n, x), v| (*v.min(&n), *v.max(&x)));

    let (fuel, base) = (min_v..=max_v).fold((i32::MAX, None), |(a, b), base| {
        let fuel = total_fuel(&vs, base);
        if a > fuel {
            (fuel, Some(base))
        } else {
            (a, b)
        }
    });
    (fuel, base.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_find_min_base() {
        let vs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(find_min_base(&vs, total_fuel1), (37, 2));
    }

    #[test]
    fn run_y2021_d7_q1() {
        let text = read_file("data/2021/input7.txt");
        let vs = split_text::<i32>(text.as_str().trim(), ',');
        assert_eq!(find_min_base(&vs, total_fuel1), (352997, 317));
    }

    #[test]
    fn run_y2021_d7_q2() {
        let text = read_file("data/2021/input7.txt");
        let vs = split_text::<i32>(text.as_str().trim(), ',');
        assert_eq!(find_min_base(&vs, total_fuel2), (101571302, 466));
    }

    #[test]
    fn test_total_fuel1() {
        let vs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(total_fuel1(&vs, 2), 37);
    }
}
