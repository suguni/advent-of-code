pub fn max_x_dist(start_vel: i32) -> i32 {
    start_vel * (start_vel + 1) / 2
}

pub fn range_of_x_vel(left: i32, right: i32) -> (i32, i32) {
    let l = (((left * 8 + 1) as f32).sqrt() - 1.0) / 2.0;
    let r = (((right * 8 + 1) as f32).sqrt() - 1.0) / 2.0;
    (l.ceil() as i32, r.floor() as i32)
}

pub fn range_of_y_vel(top: i32, bottom: i32) -> (i32, i32) {
    (top.abs(), bottom.abs() - 1)
}

pub fn quiz1(bottom: i32) -> i32 {
    let bottom = bottom.abs();
    (bottom - 1) * bottom / 2
}

pub fn dist(mut vel: i32, step: i32) -> i32 {
    let mut distance = 0;
    for _ in 0..step {
        distance += vel;
        vel -= 1;
        if vel == 0 {
            break;
        }
    }
    distance
}

pub fn sum_of_n(m: i32, c: i32) -> i32 {
    sum_of(m - c, m)
}

fn sum_of(n: i32, m: i32) -> i32 {
    m * (m + 1) / 2 - n * (n + 1) / 2
}
/*
pub fn x_steps(tx: i32) -> Vec<(i32, i32)> {
    let mut xs = vec![];

    for step in 1.. {
        // total_move = step * initial_vel - step * (step - 1) / 2;
        // if total_move == tx { got it }

        let x = tx + step * (step - 1) / 2;
        let vel = x / step;

        if (i + 1) * (i + 1) > tx {
            break;
        }

        if x % (i + 1) == 0 {
            xs.push((n, i + 1));
        }
    }

    xs
}
pub fn y_path(ty: i32) -> Vec<(i32, i32)> {
    let mut vel = 1;

    while vel <= ty {
        let mut dist = 0;
        for step in i.. {
            dist += step;
            if dist == ty {}
        }
    }

    let mut xs = vec![];

    for i in 0.. {
        let x = tx + (i * (i + 1) / 2);
        let n = x / (i + 1);

        if (i + 1) * (i + 1) > tx {
            break;
        }

        if x % (i + 1) == 0 {
            xs.push((i + 1, n));
        }
    }

    xs
}

*/
#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: (i32, i32, i32, i32) = (70, 125, -121, -159);

    #[test]
    fn test_dist() {
        assert_eq!(dist(10, 1), 10);
        assert_eq!(dist(10, 2), 19);
        assert_eq!(dist(10, 3), 27);
    }

    #[test]
    fn test_range_of_x_vel() {
        assert_eq!(range_of_x_vel(20, 30), (6, 7));
    }

    #[test]
    fn run_quiz1() {
        assert_eq!(quiz1(-10), 45);
        assert_eq!(quiz1(TARGET.3), 12561);
    }
}
