use std::collections::HashSet;

fn prime_factorize(num: i32) -> (Vec<(i32, u32)>, Vec<i32>) {
    let mut n = num;
    let mut primes = Vec::new();
    let mut divisors: HashSet<i32> = HashSet::new();
    divisors.insert(1);
    divisors.insert(num);

    let mut count = 0;
    let mut multi = 2;

    while n % 2 == 0 && n > 1 {
        divisors.insert(multi);
        divisors.insert(num / multi);
        multi *= 2;
        n /= 2;
        count += 1;
    }

    if count > 0 {
        primes.push((2, count));
    }

    let mut denominator = 3;
    let mut multi = denominator;
    let mut count = 0;

    while n > 1 {
        if n % denominator == 0 {
            divisors.insert(multi);
            divisors.insert(num / multi);
            multi *= denominator;
            n /= denominator;
            count += 1;
        } else {
            if count > 0 {
                primes.push((denominator, count));
                count = 0;
            }
            denominator += 2;
            multi = denominator;
        }
    }

    if count > 0 {
        primes.push((denominator, count));
    }

    let mut ds: Vec<i32> = divisors.into_iter().collect();
    ds.sort();
    (primes, ds)
}

fn sum_of_factors(factors: &Vec<(i32, u32)>) -> i32 {
    factors.iter()
        .map(|(k, v)|
            (0..=*v).map(|v| k.pow(v)).sum::<i32>())
        .product()
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn quiz1() {
        let mut house = 2;
        let (_sum, r) = loop {
            let (factors, _) = prime_factorize(house);
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

        // 50 ~ 1
        // 100 ~ 1+2
        // 150 ~ 1+2+3
        // 200 ~ 1+2+3+4


        // 3 번 까지만 사용

        //0 1 1
        //0 2 1 2
        //0 3 1 3
        //1 4 _ 2 4     2^0 + 2^1 + 2^2 - 1
        //1 5 _ 5       5^0 + 5^1 - 1
        //1 6 _ 2 3     (2^0 + 2^1) * (3^0 + 3^1) - 1
        //2 7 _ 7       7^0 + 7^1 - 1
        //2 8 _ _ 4 8   (2^0 + 2^1 + 2^2 + 2^3) - (1 + 2)
        //2 9 _ 3 9
        //3 10 _ _ 5 10   (2^0+2^1)*(5^0+5^1) - (1 + 2)
        //3 11 _ 11       (11^0+11^1) - 1
        //3 12 _ _ _ 4 6 12 28   (2*2*3) = (2^0+2^1+2^2) * (3^0+3^1) - (1+2+3)
        //4 13 _ 13      (13^0 + 13^1) - 1
        //4 14 _ _ 7 17  (2^0+2^1)*(7*0+7^1) - (1+2)
        //4 15 _ _ 5 15  (3^0+3^1)*(5*0+5^1) - (1+3)
        //5 16 _ _ _ 8 16  2-2-2-2 (2^0+2^1+2^2+2^3) - (1+2+4)
        //5 17 _ 17
        //5 18 _ _ _ 6 9 18  2-3-3 (2^0+2^1)*(3*0+3^1+3^2) - (1+2+3)
        //6 19 1 19
        //6 20 1 2 4 5 10 20  2-2-5 (2^0+2^1+2^2)*(5*0+5^1) - (1+2+4+5)

        // 786240 3413760 191 37549259
        // 776160 3361176 147 36971319

        let mut house = 776160; // 2; 776160
        let (_sum, r) = loop {

            let (factors, divisors) = prime_factorize(house);
            let subtraction: i32 = divisors
                .into_iter()
                .take_while(|d| *d < ((house - 1) / 50))
                .sum();

            let presents = sum_of_factors(&factors);
            let sum = (presents - subtraction) * 11;
            println!("{} {} {} {}", house, presents, subtraction, sum);
            if sum >= 36000000 {
                break (sum, house);
            }
            house += 1;
        };

        assert_eq!(r, 776160);
    }

    #[test]
    fn test_sum_of_factors() {
        assert_eq!(sum_of_factors(
            &vec![(2, 2), (3, 3)]
        ), (2_i32.pow(0)+2_i32.pow(1)+2_i32.pow(2))*(3_i32.pow(0)+3_i32.pow(1)+3_i32.pow(2)+3_i32.pow(3)));
    }

    #[test]
    fn test_prime_factors() {
        let (factors, divisors) = prime_factorize(2);
        assert_eq!(factors, vec![(2, 1)]);
        assert_eq!(divisors, vec![1, 2]);

        let (factors, divisors) = prime_factorize(3);
        assert_eq!(factors, vec![(3, 1)]);
        assert_eq!(divisors, vec![1, 3]);

        let (factors, divisors) = prime_factorize(4);
        assert_eq!(factors, vec![(2, 2)]);
        assert_eq!(divisors, vec![1, 2, 4]);

        let (factors, divisors) = prime_factorize(6);
        assert_eq!(factors, vec![(2, 1), (3, 1)]);
        assert_eq!(divisors, vec![1, 2, 3, 6]);

        let (factors, divisors) = prime_factorize(2*2*3*5);
        assert_eq!(factors, vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(divisors, vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);  // 60

        assert_eq!(prime_factorize(2 * 2 * 2 * 3 * 3 * 5 * 5 * 7 * 7).0,
                   vec![(2, 3), (3, 2), (5, 2), (7, 2)]);
    }
}
