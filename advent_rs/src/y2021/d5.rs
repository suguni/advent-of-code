use regex::Regex;

// FIXME min_max_coords 에서 min 이 항상 0 이 나오는 오류 존재한다.
// line 의 좌표를 무조건 0 기준으로 계산하므로 위 오류때문에 이후 로직이 문제가 없었음.
// min_max_coords 함수를 만든 의도와는 다르게 잘못 동작하지만 계산은 맞고 있던 상황.
// 문제는 불필요하게 보드의 크기를 크게 만들고 있다는 점과 코드 작성 의도와 동작이 다르다는 점.
// 나중에 수정?

type Line = ((i32, i32), (i32, i32));

fn parse_line(line: &str) -> Line {
    let cas = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$")
        .unwrap()
        .captures(line)
        .unwrap();

    (
        (
            String::from(&cas[1]).parse::<i32>().unwrap(),
            String::from(&cas[2]).parse::<i32>().unwrap(),
        ),
        (
            String::from(&cas[3]).parse::<i32>().unwrap(),
            String::from(&cas[4]).parse::<i32>().unwrap(),
        ),
    )
}

fn min_max_coords(lines: &Vec<Line>) -> ((i32, i32), (i32, i32)) {
    lines.iter().fold(
        ((0, 0), (0, 0)),
        |((min_x, min_y), (max_x, max_y)), ((ax, ay), (bx, by))| {
            let new_min_x = min_x.min(*ax).min(*bx);
            let new_min_y = min_y.min(*ay).min(*by);
            let new_max_x = max_x.max(*ax).max(*bx);
            let new_max_y = max_y.max(*ay).max(*by);
            ((new_min_x, new_min_y), (new_max_x, new_max_y))
        },
    )
}

fn is_hv_line(((sx, sy), (ex, ey)): &Line) -> bool {
    sx == ex || sy == ey
}

fn filter_hv_lines(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| is_hv_line(*line))
        .map(|line| *line)
        .collect()
}

fn draw_line(board: &mut Vec<u32>, width: usize, ((sx, sy), (ex, ey)): &Line) {
    let (dx, dy, steps) = if sx == ex {
        if ey > sy {
            (0, 1, ey - sy)
        } else {
            (0, -1, sy - ey)
        }
    } else if sy == ey {
        if ex > sx {
            (1, 0, ex - sx)
        } else {
            (-1, 0, sx - ex)
        }
    } else {
        if (ey - sy) / (ex - sx) > 0 {
            if ex > sx {
                (1, 1, ex - sx)
            } else {
                (-1, -1, sx - ex)
            }
        } else {
            if ex > sx {
                (1, -1, ex - sx)
            } else {
                (-1, 1, sx - ex)
            }
        }
    };
    (0..=steps).for_each(|s| {
        let loc = ((sy + s * dy) * width as i32 + (sx + s * dx)) as usize;
        board[loc] += 1;
    })
}

fn generate_board(lines: &Vec<Line>) -> (Vec<u32>, usize, usize) {
    let ((sx, sy), (ex, ey)) = min_max_coords(&lines);
    let width = (ex - sx + 1) as usize;
    let height = (ey - sy + 1) as usize;
    (vec![0; width * height], width, height)
}

fn draw_lines(board: &mut Vec<u32>, width: usize, lines: &Vec<Line>) {
    lines.iter().for_each(|line| draw_line(board, width, line));
}

pub fn quiz1(text: &str) -> usize {
    let lines = text.lines().map(parse_line).collect::<Vec<Line>>();
    let lines = filter_hv_lines(&lines);
    let (mut board, width, _height) = generate_board(&lines);
    draw_lines(&mut board, width, &lines);
    board.iter().filter(|c| **c > 1).count()
}

pub fn quiz2(text: &str) -> usize {
    let lines = text.lines().map(parse_line).collect::<Vec<Line>>();
    let (mut board, width, _height) = generate_board(&lines);
    draw_lines(&mut board, width, &lines);
    board.iter().filter(|c| **c > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    const LINES: [Line; 10] = [
        ((0, 9), (5, 9)),
        ((8, 0), (0, 8)),
        ((9, 4), (3, 4)),
        ((2, 2), (2, 1)),
        ((7, 0), (7, 4)),
        ((6, 4), (2, 0)),
        ((0, 9), (2, 9)),
        ((3, 4), (1, 4)),
        ((0, 0), (8, 8)),
        ((5, 5), (8, 2)),
    ];

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("0,9 -> 5,9"), ((0, 9), (5, 9)));
    }

    #[test]
    fn test_min_max_coord() {
        assert_eq!(min_max_coords(&LINES.to_vec()), ((0, 0), (9, 9)));
    }

    #[test]
    fn test_is_hv_line() {
        assert!(is_hv_line(&LINES[0]));
        assert!(!is_hv_line(&LINES[5]));
    }

    #[test]
    fn test_draw_line() {
        let mut board = vec![0; 100];

        draw_line(&mut board, 10, &LINES[0]);
        assert_eq!(board[9 * 10..9 * 10 + 5 + 1], [1; 6]);

        draw_line(&mut board, 10, &LINES[6]);
        assert_eq!(board[9 * 10..9 * 10 + 5 + 1], [2, 2, 2, 1, 1, 1]);
    }

    #[test]
    fn test_generate_board() {
        let lines = filter_hv_lines(&LINES.to_vec());
        assert_eq!(lines.len(), 6);

        let (board, width, height) = generate_board(&lines);

        assert_eq!(board, vec![0; 100]);
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }

    #[test]
    fn test_draw_lines() {
        let lines = filter_hv_lines(&LINES.to_vec());
        let mut board = vec![0; 100];
        draw_lines(&mut board, 10, &lines);

        assert_eq!(
            board,
            vec![
                0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 2, 1, 1, 1, 2, 1, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn run_quiz1() {
        let text = read_file("data/2021/input5.txt");
        assert_eq!(quiz1(text.as_str()), 5294);
    }

    #[test]
    fn test_draw_all_lines() {
        let lines = LINES.to_vec();
        let mut board = vec![0; 100];
        draw_lines(&mut board, 10, &lines);

        assert_eq!(
            board,
            vec![
                1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 1, 0, 1, 1,
                1, 0, 0, 0, 0, 1, 0, 2, 0, 2, 0, 0, 0, 1, 1, 2, 3, 1, 3, 2, 1, 1, 0, 0, 0, 1, 0, 2,
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 1, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn run_quiz2() {
        let text = read_file("data/2021/input5.txt");
        assert_eq!(quiz2(text.as_str()), 21698);
    }
}
