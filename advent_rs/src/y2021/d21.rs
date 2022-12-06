fn next_pos(pos: usize, rolls: [usize; 3]) -> usize {
    (pos + rolls[0] + rolls[1] + rolls[2] - 1) % 10 + 1
}

fn play(mut p1_pos: usize, mut p2_pos: usize, target_score: usize) -> usize {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut roll = 0;
    let mut count = 0;

    loop {
        roll += 1;
        let r1 = (roll - 1) % 100 + 1;
        roll += 1;
        let r2 = (roll - 1) % 100 + 1;
        roll += 1;
        let r3 = (roll - 1) % 100 + 1;
        count += 3;
        p1_pos = next_pos(p1_pos, [r1, r2, r3]);
        p1_score += p1_pos;
        if p1_score >= target_score {
            return count * p2_score;
        }

        roll += 1;
        let r1 = (roll - 1) % 100 + 1;
        roll += 1;
        let r2 = (roll - 1) % 100 + 1;
        roll += 1;
        let r3 = (roll - 1) % 100 + 1;
        count += 3;
        p2_pos = next_pos(p2_pos, [r1, r2, r3]);
        p2_score += p2_pos;
        if p2_score >= target_score {
            return count * p1_score;
        }
    }
}

pub fn universe() -> Vec<[usize; 3]> {
    let mut univ = vec![];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                univ.push([i as usize + 1, j as usize + 1, k as usize + 1]);
            }
        }
    }
    univ
}

pub fn quiz1() -> usize {
    play(1, 2, 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_pos() {
        assert_eq!(next_pos(4, [1, 2, 3]), 10);
        assert_eq!(next_pos(8, [4, 5, 6]), 3);
        assert_eq!(next_pos(10, [7, 8, 9]), 4);
    }

    #[test]
    fn test_play() {
        assert_eq!(play(4, 8, 1000), 739785);
    }

    #[test]
    fn run_y2021_d21_quiz1() {
        assert_eq!(quiz1(), 598416);
    }

    #[test]
    fn test_ok() {
        assert!(true);
        // assert_eq!(341960390180808_u64 + 444356092776315_u64, 0);
        // assert_eq!(786316482957123_u64, 3_u64.pow(32));
        assert_eq!(universe(), [[0, 0, 0]; 27]);
    }
}
