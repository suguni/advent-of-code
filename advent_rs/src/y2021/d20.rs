type Image = Vec<Vec<u8>>;

pub fn load_data(text: &str) -> (Vec<u8>, Image) {
    let (t1, t2) = text.split_once("\n\n").unwrap();
    (load_pixels(t1), load_image(t2))
}

pub fn load_pixels(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => {
                panic!()
            }
        })
        .collect()
}

pub fn load_image(text: &str) -> Image {
    text.lines().map(|line| load_pixels(line)).collect()
}

pub fn pad_image(img: &Image, pad: u8) -> Image {
    let mut padded = Vec::new();
    let cols = img[0].len() + 2;

    padded.push(vec![pad; cols]);
    for row in img {
        let mut r = Vec::with_capacity(cols);
        r.push(pad);
        r.extend(row);
        r.push(pad);
        padded.push(r);
    }
    padded.push(vec![pad; cols]);

    padded
}

pub fn index(img: &Image, row: usize, col: usize, pad: u8) -> usize {
    let mut index = 0_usize;
    let rows = img.len() as i16;
    let cols = img[0].len() as i16;

    for r in 0..3 {
        for c in 0..3 {
            let rr = row as i16 + r - 1;
            let cc = col as i16 + c - 1;

            let v = if rr >= 0 && rr < rows && cc >= 0 && cc < cols {
                img[rr as usize][cc as usize]
            } else {
                pad
            };

            index = index * 2 + v as usize;
        }
    }
    index
}

pub fn enhance_image(img: &Image, algo: &Vec<u8>, pad: u8) -> Image {
    let padded = pad_image(img, pad);
    let rows = padded.len();
    let cols = padded[0].len();

    let mut result = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row = vec![0; cols];
        for c in 0..cols {
            let index = index(&padded, r, c, pad);
            row[c] = algo[index];
        }
        result.push(row);
    }
    result
}

pub fn count_white(img: &Image) -> usize {
    let mut c = 0;
    for row in img {
        c += row.iter().filter(|c| **c == 1).count();
    }
    c
}

pub fn print_image(img: &Image) {
    for row in img {
        row.iter().for_each(|v| {
            let c = if *v == 0 { '.' } else { '#' };
            print!("{}", c);
        });
        print!("\n");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    const IMAGE: &str = "#..#.
#....
##..#
..#..
..###";

    const ENH1: &str = ".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.";

    const ENH2: &str = ".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..";

    const ALGO: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

    #[test]
    fn test_ok() {
        assert!(true);
    }

    #[test]
    fn test_load_pixels() {
        let algo = load_pixels(ALGO);
        assert_eq!(algo.len(), 512);
        assert_eq!(algo[..10], [0, 0, 1, 0, 1, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_load_image() {
        let img = load_image(IMAGE);
        assert_eq!(img.len(), 5);
        assert_eq!(img[0].len(), 5);
    }

    #[test]
    fn test_pad_image() {
        let img = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        let padded = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(pad_image(&img, 0), padded);
    }

    #[test]
    fn test_index() {
        let image = load_image(IMAGE);
        assert_eq!(index(&image, 2, 2, 0), 34);
    }

    #[test]
    fn test_sample() {
        let image = load_image(IMAGE);
        let algo = load_pixels(ALGO);

        let image = enhance_image(&image, &algo, 0);
        assert_eq!(image, load_image(ENH1));

        let image = enhance_image(&image, &algo, 0);
        assert_eq!(image, load_image(ENH2));

        assert_eq!(count_white(&image), 35);
    }

    #[test]
    fn run_quiz1() {
        let text = read_file("data/2021/input20.txt");
        let (algo, image) = load_data(text.as_str());

        // algorithm 데이터가 모두 0이면 1, 모두 1이면 0이라 반복하도록 하드코딩, quiz2도 동일

        let image = enhance_image(&image, &algo, 0);
        let image = enhance_image(&image, &algo, 1);
        assert_eq!(count_white(&image), 5306);
    }

    #[test]
    fn run_quiz2() {
        let text = read_file("data/2021/input20.txt");
        let (algo, mut image) = load_data(text.as_str());

        for i in 0..50 {
            image = enhance_image(&image, &algo, i % 2);
        }

        assert_eq!(count_white(&image), 17497);
    }
}
