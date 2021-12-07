use std::collections::HashSet;

fn tick(state: &mut Vec<i32>) -> usize {
    let min = *state.iter().min().unwrap();
    let mut new_generation = vec![];

    for s in state.iter_mut() {
        *s -= min + 1;
        if *s < 0 {
            *s = 6;
            new_generation.push(8);
        }
    }

    if !new_generation.is_empty() {
        state.append(&mut new_generation);
    }

    (min + 1) as usize
}

pub fn play(state: &mut Vec<i32>, mut count: usize) -> usize {
    while count != 0 {
        count -= tick(state);
        println!("{}", count);
    }
    state.len()
}

// 6
// 5
// 4
// 3
// 2
// 1
// 0
// 6 8
// 5 7
// 4 6
// 3 5
// 2 4
// 1 3
// 0 2
// 6 1 8
// 5 0 7
// 4 6 6 8
// 3 5 5 7
// 2 4 4 6
// 1 3 3 5
// 0 2 2 4
// 6 1 1 3 8
// 5 0 0 2 7
// 4 6 6 1 6 8 8

// 24

pub fn live_fishes_count(starts: &Vec<i64>, days: i64) -> i64 {
    let unique_starts: HashSet<i64> = starts.iter().cloned().collect();
    let min = *unique_starts.iter().min().unwrap();
    let mut cache = vec![-1; align_days(min, days) as usize + 1];

    let mut counts = [0; 7];

    for start in unique_starts.iter() {
        counts[*start as usize] = live_born_fish_count(align_days(*start, days), &mut cache) + 1;
    }

    starts.iter().map(|i| counts[*i as usize]).sum()
}

fn align_days(start: i64, days: i64) -> i64 {
    days + 6 - start
}

fn live_born_fish_count(days: i64, cache: &mut Vec<i64>) -> i64 {
    if days < 7 {
        return 0;
    }

    if cache[days as usize] >= 0 {
        return cache[days as usize];
    }

    let n = days / 7;

    let mut count = n;

    for i in 1..=n {
        let d = days - 7 * i - 2;

        if d < 0 {
            break;
        }

        count += live_born_fish_count(d, cache);
    }

    cache[days as usize] = count;
    count
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{read_file, split_text};

    fn assert_live_born_fish_count(start: i64, days: i64, expected: i64) {
        let days = align_days(start, days);
        let size = days as usize + 1;
        assert_eq!(live_born_fish_count(days, &mut vec![-1; size]), expected);
    }

    #[test]
    fn test_born_count_start() {
        assert_live_born_fish_count(6, 6, 0);
        assert_live_born_fish_count(6, 7, 1);
        assert_live_born_fish_count(6, 14, 2);
        assert_live_born_fish_count(6, 16, 3);
        assert_live_born_fish_count(1, 1, 0);
        assert_live_born_fish_count(1, 2, 1);
        assert_live_born_fish_count(1, 9, 2);
        assert_live_born_fish_count(1, 11, 3);

        // 3
        // 2
        // 1
        // 0
        // 6 8
        // 5 7
        // 4 6
        // 3 5
        // 2 4
        // 1 3
        // 0 2
        // 6 1 8
        // 5 0 7
        // 4 6 6 8
        // 3 5 5 7
        // 2 4 4 6
        // 1 3 3 5
        // 0 2 2 4
        // 6 1 1 3 8
        assert_live_born_fish_count(3, 18, 4);

        // 4
        // 3
        // 2
        // 1
        // 0
        // 6 8
        // 5 7
        // 4 6
        // 3 5
        // 2 4
        // 1 3
        // 0 2
        // 6 1 8
        // 5 0 7
        // 4 6 6 8
        // 3 5 5 7
        // 2 4 4 6
        // 1 3 3 5
        // 0 2 2 4
        assert_live_born_fish_count(4, 18, 3);

        // 2
        // 1
        // 0
        // 6 8
        // 5 7
        // 4 6
        // 3 5
        // 2 4
        // 1 3
        // 0 2
        // 6 1 8
        // 5 0 7
        // 4 6 6 8
        // 3 5 5 7
        // 2 4 4 6
        // 1 3 3 5
        // 0 2 2 4
        // 6 1 1 3 8
        // 5 0 0 2 7
        assert_live_born_fish_count(2, 17, 4);
        assert_live_born_fish_count(2, 18, 4);

        // 1
        // 0
        // 6 8
        // 5 7
        // 4 6
        // 3 5
        // 2 4
        // 1 3
        // 0 2
        // 6 1 8
        // 5 0 7
        // 4 6 6 8
        // 3 5 5 7
        // 2 4 4 6
        // 1 3 3 5
        // 0 2 2 4
        // 6 1 1 3 8
        // 5 0 0 2 7
        // 4 6 6 1 6 8 8
        assert_live_born_fish_count(1, 18, 6);
    }

    #[test]
    fn test_data1() {
        let state = vec![3, 4, 3, 1, 2];
        assert_eq!(live_fishes_count(&state, 18), 26);
        assert_eq!(live_fishes_count(&state, 80), 5934);
        assert_eq!(live_fishes_count(&state, 256), 26984457539);
    }

    #[test]
    fn test_load_data() {
        let text = "3,4,3,1,2";
        assert_eq!(split_text::<i64>(text, ','), vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_one_tick() {
        let mut state = vec![3, 4, 3, 1, 2];

        tick(&mut state);
        assert_eq!(state, [1, 2, 1, 6, 0, 8]);
    }

    #[test]
    fn test_play() {
        let mut state = vec![3, 4, 3, 1, 2];
        assert_eq!(play(&mut state, 18), 26);

        let mut state = vec![3, 4, 3, 1, 2];
        assert_eq!(play(&mut state, 80), 5934);
    }

    #[test]
    fn run_d6_quiz1() {
        let text = read_file("data/2021/input6.txt");
        let state = split_text(text.as_str().trim(), ',');
        assert_eq!(live_fishes_count(&state, 80), 380612);
    }

    #[test]
    fn run_d6_quiz2() {
        let text = read_file("data/2021/input6.txt");
        let state = split_text(text.as_str().trim(), ',');
        assert_eq!(live_fishes_count(&state, 256), 1710166656900);
    }
}
