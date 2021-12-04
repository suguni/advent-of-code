pub fn load_data(text: &str) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut chunks = text.split("\n\n");

    let order = chunks
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let boards = chunks
        .map(|board| {
            board
                .split_whitespace()
                .map(|ns| ns.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (order, boards)
}

fn mark_board(boards: &mut Vec<Vec<i32>>, value: i32) {
    for board in boards {
        for v in board.iter_mut() {
            if *v == value {
                *v = -1;
            }
        }
    }
}

fn check_win_board(board: &Vec<i32>) -> bool {
    board.chunks(5).any(|row| row.iter().all(|v| *v == -1))
        || (0..5)
            .map(|i| board[i..].iter().step_by(5))
            .any(|mut col| col.all(|v| *v == -1))
}

pub fn quiz1(text: &str) -> i32 {
    let (order, mut boards) = load_data(text);

    let mut last_number = 0;
    let mut win_board = 0;

    'outer: for n in order {
        last_number = n;
        mark_board(&mut boards, n);
        for (i, board) in boards.iter().enumerate() {
            if check_win_board(board) {
                win_board = i;
                break 'outer;
            }
        }
    }

    let s: i32 = boards[win_board].iter().filter(|v| **v != -1).sum();

    last_number * s
}

pub fn quiz2(text: &str) -> i32 {
    let (order, mut boards) = load_data(text);

    let mut last_number = -1;
    let mut last_board = None;

    for n in order {
        last_number = n;
        mark_board(&mut boards, n);

        let wins: Vec<usize> = boards
            .iter()
            .enumerate()
            .filter_map(|(i, board)| {
                if check_win_board(board) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        for win in wins.iter().rev() {
            last_board.replace(boards.remove(*win as usize));
        }

        if boards.len() == 0 {
            break;
        }
    }

    let b = last_board.unwrap();
    let s: i32 = b.iter().filter(|v| **v != -1).sum();

    last_number * s
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::read_file;

    const DATA: &str = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_load_boards() {
        let (order, boards) = load_data(DATA.trim());
        assert_eq!(order.len(), 27);
        assert_eq!(boards.len(), 3);
    }

    #[test]
    fn test_board_win() {
        assert!(check_win_board(&vec![-1; 25]));
        assert!(check_win_board(&vec![
            0, 0, -1, 0, 0, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!check_win_board(&vec![0; 25]));
        assert!(check_win_board(&vec![
            0, 0, -1, 0, 0, -1, 0, -1, -1, -1, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0
        ]));
    }

    #[test]
    fn test_quiz1() {
        let text = read_file("data/2021/input4.txt");
        assert_eq!(quiz1(text.as_str()), 32844);
    }

    #[test]
    fn test_quiz2() {
        let text = read_file("data/2021/input4.txt");
        assert_eq!(quiz2(text.as_str()), 4920);
    }
}
