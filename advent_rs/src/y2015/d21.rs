use std::iter::FromIterator;

#[derive(Copy, Clone)]
struct Item(i32, i32, i32);

const WEAPONS: [Item; 5] = [
    Item(8, 4, 0),
    Item(10, 5, 0),
    Item(25, 6, 0),
    Item(40, 7, 0),
    Item(74, 8, 0),
];

const ARMORS: [Item; 5] = [
    Item(13, 0, 1),
    Item(31, 0, 2),
    Item(53, 0, 3),
    Item(75, 0, 4),
    Item(102, 0, 5),
];

const RINGS: [Item; 6] = [
    Item(25, 1, 0),
    Item(50, 2, 0),
    Item(100, 3, 0),
    Item(20, 0, 1),
    Item(40, 0, 2),
    Item(80, 0, 3),
];

const BOSS: Player = Player { hit_points: 104, damage: 8, armor: 1, items: String::new() };

fn combinations(ns: &Vec<usize>, k: usize) -> Vec<Vec<usize>> {
    let mut result = vec![vec![]];

    if k == 0 {
        return result;
    } else {
        for i in 0..ns.len() {
            let mut nss = ns.clone();
            let v = nss.remove(i);

            let combs = combinations(&nss, k - 1);
            for mut vs in combs {
                vs.push(v);
                vs.sort();
                if !result.contains(&vs) {
                    result.push(vs);
                }
            }
        }
    }

    result
}

fn players() -> Vec<(i32, Player)> {
    let mut result = vec![];

    for i in 0..WEAPONS.len() {
        let jss = combinations(
            &Vec::from_iter(0..ARMORS.len()), 1);

        for js in jss {

            let kss = combinations(
                &Vec::from_iter(0..RINGS.len()), 2);

            for ks in kss {

                let mut cost = 0;
                let mut damage = 0;
                let mut armor = 0;
                let mut items = String::new();

                cost += WEAPONS[i].0;
                damage += WEAPONS[i].1;
                armor += WEAPONS[i].2;
                items.push_str(&format!("W{}", i+1));

                for j in js.iter() {
                    cost += ARMORS[*j].0;
                    damage += ARMORS[*j].1;
                    armor += ARMORS[*j].2;
                    items.push_str(&format!("A{}", j+1));
                }

                for k in ks {
                    cost += RINGS[k].0;
                    damage += RINGS[k].1;
                    armor += RINGS[k].2;
                    items.push_str(&format!("R{}", k+1));
                }

                result.push((cost, Player::new(100, damage, armor, items)));
            }
        }
    }

    result
}

#[derive(Clone, Debug)]
struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
    items: String,
}

impl Player {
    fn new(hit_points: i32, damage: i32, armor: i32, items: String) -> Self {
        Self { hit_points, damage, armor, items }
    }

    fn is_win(&self, other: &Player) -> bool {
        let my_attack = (self.damage - other.armor).max(1);
        let other_attack = (other.damage - self.armor).max(1);

        let other_kill_count = (other.hit_points as f32 / my_attack as f32).ceil() as i32;
        let my_kill_count = (self.hit_points as f32 / other_attack as f32).ceil() as i32;

        if my_kill_count > other_kill_count {
            true
        } else if my_kill_count < other_kill_count {
            false
        } else {
            let other_rem = other.hit_points % my_attack;
            let my_rem = self.hit_points % other_attack;
            if (my_rem == 0 && other_rem == 0) || (my_rem != 0 && other_rem != 0) {
                true
            } else {
                other_rem == 0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiz1() {
        let mut players = players();
        players.sort_by(|p1, p2| (*p1).0.cmp(&((*p2).0)));

        let (c, p) = players
                .into_iter()
                .skip_while(|(_, p)| !p.is_win(&BOSS))
                .nth(0)
                .unwrap();

        println!("{} {:?}", c, p);
        assert_eq!(c, 78);
    }

    #[test]
    fn quiz2() {
        let mut players = players();
        players.sort_by(|p1, p2| (*p2).0.cmp(&((*p1).0)));

        let (c, p) = players
            .into_iter()
            .skip_while(|(_, p)| p.is_win(&BOSS))
            .nth(0)
            .unwrap();

        println!("{} {:?}", c, p);
        assert_eq!(c, 148);
    }

    #[test]
    fn test_combination() {
        let input = vec![1, 2, 3];
        let mut output = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![2, 3],
            vec![1, 3],
        ];
        output.sort();
        assert_eq!(combinations(&input, 2), output);
    }

    #[test]
    fn test_win() {
        let you = Player { hit_points: 100, damage: 9, armor: 0, items: "W1R2R3".to_string() };
        let boss = Player { hit_points: 104, damage: 8, armor: 1, items: "".to_string() };
        assert!(you.is_win(&boss));

        let you = Player::new(8, 5, 5, "".to_string());
        let boss = Player::new(12, 7, 2, "".to_string());
        assert!(you.is_win(&boss));

        let you = Player::new(8, 5, 5, "".to_string()); // 3
        let boss = Player::new(13, 7, 2, "".to_string());  // 2
        assert!(!you.is_win(&boss));

        let you = Player { hit_points: 100, damage: 7, armor: 3, items: "W4A2R4".to_string() };
        assert!(you.is_win(&BOSS));

        let you = Player::new(14, 2, 0, "".to_string());
        let boss = Player::new(15, 2, 0, "".to_string());
        assert!(!you.is_win(&boss));

        let you = Player::new(15, 2, 0, "".to_string());
        let boss = Player::new(14, 2, 0, "".to_string());
        assert!(you.is_win(&boss));

        let you = Player::new(14, 2, 0, "".to_string());
        let boss = Player::new(14, 2, 0, "".to_string());
        assert!(you.is_win(&boss));
    }
}