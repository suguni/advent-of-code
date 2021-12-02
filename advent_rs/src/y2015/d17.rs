// TODO Iterator implementations

// https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-11.html#%_idx_728

fn combinations_n(amount: i32, coins: &Vec<i32>) -> usize {
    if amount == 0 {
        1
    } else if amount < 0 || coins.is_empty() {
        0
    } else {
        combinations_n(amount, &coins[1..].to_vec())
            + combinations_n(amount - coins[0], &coins[1..].to_vec())
    }
}

fn combinations_d(amount: i32, coins: &Vec<i32>) -> Vec<Vec<i32>> {
    if amount == 0 {
        vec![Vec::new()]
    } else if amount < 0 || coins.is_empty() {
        Vec::new()
    } else {
        let mut l = combinations_d(amount, &coins[1..].to_vec());
        let mut r = combinations_d(amount - coins[0], &coins[1..].to_vec());
        for v in &mut r {
            v.insert(0, coins[0]);
        }
        l.append(&mut r);
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn quiz1() {
        let containers: Vec<i32> = read_file("../data/2015/input17.txt")
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        assert_eq!(combinations_n(150, &containers), 4372);
    }

    #[test]
    fn quiz2() {
        let containers: Vec<i32> = read_file("../data/2015/input17.txt")
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        let combs = combinations_d(150, &containers);
        let min_size = combs.iter().min_by_key(|vs| vs.len()).unwrap().len();

        let count = combs.iter().filter(|vs| vs.len() == min_size).count();

        assert_eq!(count, 0);
    }

    #[test]
    fn test_simple_combinations2() {
        let containers = vec![20];
        let store = 20;

        let combs = combinations_n(store, &containers);
        assert_eq!(combs, 1);
    }

    #[test]
    fn test_combinations() {
        let containers = vec![20, 15, 10, 5, 5];
        let store = 25;

        let combs = combinations_n(store, &containers);
        assert_eq!(combs, 4);
    }

    #[test]
    fn test_combinations_d() {
        let containers = vec![20, 15, 10, 5, 5];
        let store = 25;

        let combs = combinations_d(store, &containers);
        assert_eq!(combs.len(), 4);

        assert!(combs.contains(&vec![15, 10]));
        assert!(combs.contains(&vec![20, 5]));
        assert!(combs.contains(&vec![15, 5, 5]));
    }
}
