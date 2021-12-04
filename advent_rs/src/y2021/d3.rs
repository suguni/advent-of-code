use crate::read_file;

fn load_bits(text: &str) -> Vec<Vec<u8>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn most_common_value(data: &Vec<Vec<u8>>, indices: &Vec<usize>, pos: usize) -> i8 {
    let mut sum = 0;
    for index in indices {
        sum += if data[*index][pos] == 0 { -1 } else { 1 };
    }
    if sum > 0 {
        1
    } else if sum < 0 {
        0
    } else {
        -1
    }
}

pub fn quiz1() -> i32 {
    let text = read_file("data/2021/input3.txt");
    let data = load_bits(text.as_str());
    let size = data[0].len();
    let indices = (0..data.len()).collect::<Vec<usize>>();

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in 0..size {
        let c = most_common_value(&data, &indices, bit);
        let (g, e) = match c {
            1 => (1, 0),
            0 => (0, 1),
            _ => panic!(),
        };

        gamma = gamma * 2 + g;
        epsilon = epsilon * 2 + e;
    }

    gamma * epsilon
}

fn find_rating(data: &Vec<Vec<u8>>, bit_matcher: fn(i8) -> u8) -> Vec<u8> {
    let mut indices = (0..data.len()).collect::<Vec<usize>>();
    let size = data[0].len();

    let mut bit = 0;
    loop {
        let c = bit_matcher(most_common_value(&data, &indices, bit));

        indices.retain(|i| {
            let bits = &data[*i];
            bits[bit] == c
        });

        if indices.len() == 1 {
            let bits = &data[indices[0]];
            break bits.clone();
        }

        bit += 1;
        bit %= size;
    }
}

pub fn quiz2() -> i32 {
    let text = read_file("data/2021/input3.txt");
    let data = load_bits(text.as_str());

    let oxy = find_rating(&data, |c| if c == -1 { 1 } else { c as u8 });
    let co2 = find_rating(&data, |c| if c == -1 || c == 1 { 0 } else { 1 });

    let oxy: i32 = oxy.iter().fold(0, |acc, v| acc * 2 + *v as i32);
    let co2: i32 = co2.iter().fold(0, |acc, v| acc * 2 + *v as i32);
    oxy * co2
}

#[cfg(test)]
mod tests {

    use super::*;

    const DATA: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(), 4138664);
    }

    #[test]
    fn test_most_common_value() {
        let vs = vec![vec![1, 0, 1, 1, 0], vec![1, 0, 1, 1, 1]];
        assert_eq!(most_common_value(&vs, &vec![0, 1], 4), -1);
        assert_eq!(most_common_value(&vs, &vec![0, 1], 3), 1);
    }

    #[test]
    fn test_find_ratings() {
        let data = load_bits(DATA.trim());

        let oxy = find_rating(&data, |c| if c == -1 { 1 } else { c as u8 });
        assert_eq!(oxy, vec![1, 0, 1, 1, 1]);

        let co2 = find_rating(&data, |c| if c == -1 || c == 1 { 0 } else { 1 });
        assert_eq!(co2, vec![0, 1, 0, 1, 0]);
    }

    #[test]
    fn run_quiz2() {
        assert_eq!(quiz2(), 4273224);
    }
}
