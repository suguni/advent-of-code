pub const L: i32 = -1;
pub const R: i32 = -2;
pub const C: i32 = -3;

type FishNum = Vec<i32>;

pub fn to_fish_num(text: &str) -> FishNum {
    text.chars().fold(Vec::new(), |mut acc, c| {
        let len = acc.len();
        match c {
            ',' => acc.push(C),
            '[' => {
                if len == 0 {
                    acc.push(L);
                } else if acc[len - 1] == C {
                    acc[len - 1] = L
                } else {
                    acc.push(L);
                }
            }
            ']' => {
                if acc[len - 1] == C {
                    acc[len - 1] = R;
                } else {
                    acc.push(R)
                }
            }
            x if x.is_digit(10) => {
                let n = x.to_digit(10).unwrap() as i32;
                if len == 0 {
                    acc.push(n)
                } else {
                    let i = len - 1;
                    if acc[i] == C {
                        acc[i] = n;
                    } else if acc[i] == L {
                        acc.push(n);
                    } else if acc[i] != L && acc[i] != R {
                        acc[i] = acc[i] * 10 + n;
                    }
                }
            }
            _ => panic!(),
        }
        acc
    })
}

pub fn add(left: &FishNum, right: &FishNum) -> FishNum {
    if left.is_empty() {
        right.to_vec()
    } else if right.is_empty() {
        left.to_vec()
    } else {
        let mut sum = vec![L];
        sum.extend(left.iter());
        sum.extend(right.iter());
        sum.push(R);
        sum
    }
}

pub fn to_string(num: &FishNum) -> String {
    num.iter()
        .fold((String::new(), String::new()), |(mut acc, last), n| {
            match *n {
                L => {
                    if last == "]" {
                        acc.push(',');
                    }
                    acc.push('[');
                    return (acc, "[".to_string());
                }
                R => {
                    acc.push(']');
                    return (acc, "]".to_string());
                }
                _ => {
                    if last != "[" {
                        acc.push(',');
                    }
                    let n = format!("{}", n);
                    acc.push_str(&n);
                    return (acc, n);
                }
            };
        })
        .0
}

pub fn explode_pair_loc(num: &FishNum) -> Option<usize> {
    let mut loc = 0;
    let mut count = 0;
    let mut max_count = 0;

    for (i, n) in num.iter().enumerate() {
        match *n {
            L => {
                count += 1;
                if count > max_count {
                    max_count = count;
                    loc = i;
                }
            }
            R => {
                count -= 1;
            }
            _ => {}
        }
    }

    if max_count > 4 {
        Some(loc)
    } else {
        None
    }
}

pub fn explode(num: &mut FishNum, loc: usize) {
    let mut s = loc;
    let left = loop {
        if num[s] != L && num[s] != R {
            break Some(s);
        }
        if s == 0 {
            break None;
        } else {
            s -= 1;
        }
    };

    s = loc + 3;

    let right = loop {
        if num[s] != L && num[s] != R {
            break Some(s);
        }
        if s == num.len() - 1 {
            break None;
        } else {
            s += 1;
        }
    };

    if let Some(left) = left {
        num[left] += num[loc + 1];
    }

    if let Some(right) = right {
        num[right] += num[loc + 2];
    }

    (0..3).for_each(|_| {
        num.remove(loc);
    });
    num[loc] = 0;
}

pub fn split_loc(num: &FishNum) -> Option<usize> {
    num.iter().position(|n| *n != L && *n != R && *n >= 10)
}

pub fn split(num: &mut FishNum, loc: usize) {
    let value = num[loc];
    let left = value / 2;
    let right = if value % 2 == 0 {
        value / 2
    } else {
        (value + 1) / 2
    };

    let pair = vec![L, left, right, R];
    num.splice(loc..(loc + 1), pair);
}

pub fn reduce(num: &mut FishNum) {
    reduce_p(num, false)
}

pub fn reduce_p(num: &mut FishNum, p: bool) {
    if p {
        println!("start {}", to_string(&num));
    }
    loop {
        let mut br = true;

        while let Some(loc) = explode_pair_loc(&num) {
            br = false;
            explode(num, loc);
            if p {
                println!("explode {}", to_string(&num));
            }
        }

        if let Some(loc) = split_loc(&num) {
            br = false;
            split(num, loc);
            if p {
                println!("split {}", to_string(&num));
            }
        }

        if br {
            break;
        }
    }
}

pub fn sum(nums: &Vec<FishNum>) -> FishNum {
    sum_p(nums, false)
}

pub fn sum_p(nums: &Vec<FishNum>, p: bool) -> FishNum {
    nums.iter().fold(Vec::new(), |acc, num| {
        let mut num = add(&acc, num);
        reduce(&mut num);
        if p {
            println!("{}", to_string(&num));
        }
        num
    })
}

pub fn load_data(text: &str) -> Vec<FishNum> {
    text.trim()
        .lines()
        .map(|line| to_fish_num(line))
        .collect::<Vec<FishNum>>()
}

fn sub_num(num: &FishNum, mut offset: usize) -> (FishNum, usize) {
    let mut ret = vec![];
    let mut left = 0;
    loop {
        let token = num[offset];
        if token == L {
            left += 1;
        } else if token == R {
            left -= 1;
        }
        ret.push(token);

        if left == 0 {
            return (ret, offset);
        }

        offset += 1;
    }
}

fn split_num(num: &FishNum) -> (FishNum, FishNum) {
    let (left, offset) = sub_num(num, 1);
    let (right, _) = sub_num(num, offset + 1);
    (left, right)
}

fn is_num(num: &FishNum) -> bool {
    num[0] != L && num[0] != R
}

pub fn magnitude(num: &FishNum) -> i32 {
    let (left, right) = split_num(num);

    let left = if is_num(&left) {
        left[0]
    } else {
        magnitude(&left)
    };

    let right = if is_num(&right) {
        right[0]
    } else {
        magnitude(&right)
    };

    3 * left + 2 * right
}

pub fn quiz2(text: &str) -> i32 {
    let nums = load_data(text);
    let len = nums.len();
    let mut perms = vec![];
    for i in 0..len {
        for j in 0..len {
            if i != j {
                perms.push((i, j));
                perms.push((j, i));
            }
        }
    }

    let mut max = 0;

    for (i, j) in perms {
        let mut n = add(&nums[i], &nums[j]);
        reduce(&mut n);
        let v = magnitude(&n);

        if v > max {
            max = v;
        }
    }
    max
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    #[test]
    fn test_magnitude() {
        assert_eq!(magnitude(&to_fish_num("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            magnitude(&to_fish_num(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )),
            4140
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            sum(&load_data(
                "[1,1]
[2,2]
[3,3]
[4,4]"
            )),
            to_fish_num("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );

        assert_eq!(
            sum(&load_data(
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
"
            )),
            to_fish_num("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        );

        assert_eq!(
            sum(&load_data(
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"
            )),
            to_fish_num("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        );

        assert_eq!(
            sum_p(
                &load_data(
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
                ),
                false
            ),
            to_fish_num("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        )
    }

    #[test]
    fn test_reduce() {
        let mut num = to_fish_num("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        reduce(&mut num);
        assert_eq!(num, to_fish_num("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        let mut num = add(
            &to_fish_num("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"),
            &to_fish_num("[[[[4,2],2],6],[8,7]]"),
        );
        reduce_p(&mut num, false);
        assert_eq!(
            num,
            to_fish_num("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_split() {
        let mut num = to_fish_num("[[[[0,7],4],[15,[0,13]]],[1,1]]");

        let loc = split_loc(&num);
        assert_eq!(loc, Some(10));

        split(&mut num, loc.unwrap());
        assert_eq!(num, to_fish_num("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
    }

    #[test]
    fn test_explode() {
        let data = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        data.iter().for_each(|(input, expected)| {
            let mut input = to_fish_num(input);
            let expected = to_fish_num(expected);
            let loc = explode_pair_loc(&input);
            explode(&mut input, loc.unwrap());
            assert_eq!(input, expected);
        });

        let mut num = to_fish_num("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

        let loc = explode_pair_loc(&num);
        explode(&mut num, loc.unwrap());
        assert_eq!(
            num,
            vec![L, L, L, L, 0, 7, R, 4, R, L, 7, L, L, 8, 4, R, 9, R, R, R, L, 1, 1, R, R]
        );

        let loc = explode_pair_loc(&num);
        explode(&mut num, loc.unwrap());
        assert_eq!(
            num,
            vec![L, L, L, L, 0, 7, R, 4, R, L, 15, L, 0, 13, R, R, R, L, 1, 1, R, R]
        );
    }

    #[test]
    fn test_to_fish_num() {
        assert_eq!(
            to_fish_num("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
            vec![
                L, L, L, L, L, 4, 3, R, 4, R, 4, R, L, 7, L, L, 8, 4, R, 9, R, R, R, L, 1, 1, R, R
            ]
        );

        assert_eq!(
            to_fish_num("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
            vec![L, L, L, L, 0, 7, R, 4, R, L, 15, L, 0, 13, R, R, R, L, 1, 1, R, R]
        )
    }

    #[test]
    fn test_print() {
        let left = vec![L, 1, 2, R];
        assert_eq!(to_string(&left), "[1,2]".to_string());
    }

    #[test]
    fn test_add() {
        let left = vec![L, 1, 2, R];
        let right = vec![L, L, 3, 4, R, 5, R];
        assert_eq!(
            add(&left, &right),
            vec![L, L, 1, 2, R, L, L, 3, 4, R, 5, R, R]
        );

        assert_eq!(add(&vec![], &right), right);
        assert_eq!(add(&left, &vec![]), left);
    }

    #[test]
    fn test_exp_pair_loc() {
        assert_eq!(
            explode_pair_loc(&to_fish_num("[[[[[1,2],[3,4]],[5,6]],[6,7]],[8,9]]")),
            Some(4)
        );
        assert_eq!(
            explode_pair_loc(&to_fish_num(
                "[[[[7,7],[[0,7],[[7,8],7]]],[[[0,8],[7,7]],9]],[[[[4,2],2],6],[8,7]]]"
            )),
            Some(13)
        );

        assert_eq!(
            explode_pair_loc(&to_fish_num("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")),
            Some(4)
        );

        assert_eq!(
            explode_pair_loc(&to_fish_num("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")),
            Some(12)
        );

        assert_eq!(
            explode_pair_loc(&to_fish_num("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            None
        );
    }

    #[test]
    fn run_y2021_d18_quiz1() {
        let text = read_file("data/2021/input18.txt");
        let num = sum(&load_data(text.as_str()));
        assert_eq!(magnitude(&num), 3892);
    }

    #[test]
    fn run_y2021_d18_quiz2() {
        let text = read_file("data/2021/input18.txt");
        assert_eq!(quiz2(text.as_str().trim()), 4909);
    }
}
