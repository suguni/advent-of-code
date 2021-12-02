fn prime_factorize(mut num: i32) -> Vec<(i32, u32)> {
    let mut primes = Vec::new();

    let mut count = 0;

    while num % 2 == 0 && num > 1 {
        num /= 2;
        count += 1;
    }

    if count > 0 {
        primes.push((2, count));
    }

    let mut denominator = 3;
    let mut count = 0;

    while num > 1 {
        if num % denominator == 0 {
            num /= denominator;
            count += 1;
        } else {
            if count > 0 {
                primes.push((denominator, count));
                count = 0;
            }
            denominator += 2;
        }
    }

    if count > 0 {
        primes.push((denominator, count));
    }

    primes
}

fn sum_of_factors(factors: &Vec<(i32, u32)>) -> i32 {
    factors
        .iter()
        .map(|(k, v)| (0..=*v).map(|v| k.pow(v)).sum::<i32>())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn quiz1() {
        let mut house = 2;
        let (_sum, r) = loop {
            let factors = prime_factorize(house);
            let presents = sum_of_factors(&factors);
            let sum = presents * 10;
            // println!("{} {} {}", house, presents, sum);
            if sum >= 36000000 {
                break (sum, house);
            }
            house += 1;
        };

        assert_eq!(r, 831600);
    }

    // #[test]
    fn quiz2() {
        // 실패한 결과부터 시도
        let mut house = 776160; // 2
        let (_sum, r) = loop {
            let factors = prime_factorize(house);
            let subtraction: i32 = (1..=(house - 1) / 50).filter(|n| house % n == 0).sum();

            let presents = sum_of_factors(&factors);
            let sum = (presents - subtraction) * 11;
            // println!("{} {} {} {}", house, presents, subtraction, sum);
            if sum >= 36000000 {
                break (sum, house);
            }
            house += 1;
        };

        assert_eq!(r, 884520);
    }

    // #[test]
    fn test_sum_of_factors() {
        assert_eq!(
            sum_of_factors(&vec![(2, 2), (3, 3)]),
            (2_i32.pow(0) + 2_i32.pow(1) + 2_i32.pow(2))
                * (3_i32.pow(0) + 3_i32.pow(1) + 3_i32.pow(2) + 3_i32.pow(3))
        );
    }

    // #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factorize(2), vec![(2, 1)]);
        assert_eq!(prime_factorize(3), vec![(3, 1)]);
        assert_eq!(prime_factorize(4), vec![(2, 2)]);
        assert_eq!(prime_factorize(6), vec![(2, 1), (3, 1)]);
        assert_eq!(prime_factorize(2 * 2 * 3 * 5), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(
            prime_factorize(2 * 2 * 2 * 3 * 3 * 5 * 5 * 7 * 7),
            vec![(2, 3), (3, 2), (5, 2), (7, 2)]
        );
    }
}
