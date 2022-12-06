use std::collections::HashSet;

use crate::permutations;

#[derive(Debug, Eq, PartialEq)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pt {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

pub fn load_data2(text: &str) -> Vec<Vec<Pt>> {
    text.split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|line| {
                    let mut s = line.split(',');
                    let x = s.next().unwrap().parse::<i32>().unwrap();
                    let y = s.next().unwrap().parse::<i32>().unwrap();
                    let z = s.next().unwrap().parse::<i32>().unwrap();
                    Pt::new(x, y, z)
                })
                .collect()
        })
        .collect()
}

pub fn load_data(text: &str) -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> {
    text.split("\n\n")
        .map(|s| {
            s.lines().skip(1).fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut xs, mut ys, mut zs), line| {
                    let mut s = line.split(',');
                    let x = s.next().unwrap().parse::<i32>().unwrap();
                    let y = s.next().unwrap().parse::<i32>().unwrap();
                    let z = s.next().unwrap().parse::<i32>().unwrap();

                    xs.push(x);
                    ys.push(y);
                    zs.push(z);

                    (xs, ys, zs)
                },
            )
        })
        .collect()
}

pub fn mm(m1: &Vec<i32>, m2: &Vec<i32>) -> Vec<i32> {
    let mut v = vec![0; 9];
    for r in 0..3 {
        for c in 0..3 {
            v[r * 3 + c] = m1[r * 3 + 0] * m2[r * 0 + c]
                + m1[r * 3 + 1] * m2[r * 1 + c]
                + m1[r * 3 + 2] * m2[r * 2 + c];
        }
    }
    v
}

pub fn rotations() -> HashSet<Vec<i32>> {
    let rx = vec![1, 0, 0, 0, 0, -1, 0, 1, 0];
    let ry = vec![0, 0, 1, 0, 1, 0, -1, 0, 0];
    let rz = vec![0, -1, 0, 1, 0, 0, 0, 0, 1];

    let mut rots = HashSet::new();
    rots.insert(rx);
    rots.insert(ry);
    rots.insert(rz);

    let perms = permutations(&rots);
    println!("{:?}", perms);

    let mut rotations = HashSet::new();

    for rots in perms.iter() {
        if rots.len() == 1 {
            rotations.insert(rots[0].clone());
        } else {
            let mut r = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
            for i in 0..rots.len() {
                r = mm(&r, &rots[i]);
            }
            rotations.insert(r);
        }
    }

    rotations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrangements() {
        // let pt = Pt::new(1, 2, 3);
        // let arrs = vec![
        //     Pt::new(1, 2, 3), // x, y, z, xy, yz, xz, xyz
        //     Pt::new(1, 3, 2),
        //     Pt::new(2, 1, 3),
        //     Pt::new(2, 3, 1),
        //     Pt::new(3, 1, 2),
        //     Pt::new(3, 2, 1),
        // ];
        // assert_eq!(pt.arrangements(), arrs);
        assert_eq!(rotations(), HashSet::new());
    }

    #[test]
    fn test_load_data2() {
        let d = load_data2(DATA);
        assert_eq!(d.len(), 5);
        assert_eq!(
            d[0][0],
            Pt {
                x: 404,
                y: -588,
                z: -901
            }
        );
    }
    #[test]
    fn test_load_data() {
        let d = load_data(DATA);
        assert_eq!(d.len(), 5);
        assert_eq!(d[0].0[0], 404);
    }

    #[test]
    fn test_ok() {
        assert!(true);
    }

    const DATA: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
}
