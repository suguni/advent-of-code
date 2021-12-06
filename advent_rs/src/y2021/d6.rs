pub fn tick(state: &mut Vec<i32>) -> usize {
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

pub fn load_data(text: &str) -> Vec<i32> {
    text.split(',').map(|s| s.parse().unwrap()).collect()
}

pub fn play(state: &mut Vec<i32>, mut count: usize) -> usize {
    while count != 0 {
        count -= tick(state);
        println!("{}", count);
    }
    state.len()
}

// pub fn jump(state: i32) -> Vec<i32> {
//     vec![6, 8]
// }

// pub fn jumps(state: &mut Vec<i32>) {
//     let min = state.iter().min();
//     for s in state.iter_mut() {
//         *s -= (min + 1);
//         if *s < 0 {
//             *s = 6;
//         }
//     }
//     state.map(|v| v - min - 1);

// }

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
// 6 1 1 3

// 6 5 4 3 2 1 0
// 6 5 4 3 2 1 0
// 6 5 4 3

// pub fn calc(state: i32, mut days: usize, cycle: usize) -> usize {
//     days -= state;
//     if days < 0 {
//         1
//     }

//     let n = days / cycle;
//     (0..=n).map(|i| 2.pow(n)).sum()
// }

#[cfg(test)]
mod tests {

    use super::*;
    use crate::read_file;

    #[test]
    fn test_load_data() {
        let text = "3,4,3,1,2";
        assert_eq!(load_data(text), vec![3, 4, 3, 1, 2]);
    }

    // #[test]
    // fn test_new_calc() {
    //     let mut states = vec![3, 4, 3, 1, 2];
    //     states.iter().map(|state| calc(state, 18, 8))
    // }

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
    fn run_quiz1() {
        let text = read_file("data/2021/input6.txt");
        let mut state = load_data(text.as_str().trim());
        assert_eq!(play(&mut state, 80), 380612);
    }

    // #[test]
    // fn run_quiz2() {
    //     let text = read_file("data/2021/input6.txt");
    //     let mut state = load_data(text.as_str().trim());
    //     assert_eq!(play(&mut state, 256), 380612);
    // }
}
