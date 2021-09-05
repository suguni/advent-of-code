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

    result.sort_by(|p1, p2| (*p1).0.cmp(&((*p2).0)));
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

    fn attack_to(&self, defender: &mut Self) {
        defender.hit_points -= (self.damage - defender.armor).max(1);
    }

    fn is_win(&self, other: &Player) -> bool {
        let turn_to_kill_other = (other.hit_points as f32 / ((self.damage - other.armor) as f32)).ceil() as i32;
        self.hit_points - (other.damage - self.armor) * (turn_to_kill_other - 1) >= 0

        // let me_damage = (self.damage - other.armor).max(1);
        // let other_damage = (other.damage - self.armor).max(1);
        // if self.hit_points / other_damage == other.hit_points / me_damage {
        //     self.hit_points % other_damage >= other.hit_points % me_damage
        // } else {
        //     self.hit_points / other_damage > other.hit_points / me_damage
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiz1() {
        let (c, p) =
            players()
                .into_iter()
                .inspect(|(c, p)| println!("{}, {:?}", c, p))
                .skip_while(|(_, p)| !p.is_win(&BOSS))
                .nth(0)
                .unwrap();

        println!("last: {}, {:?}", c, p);
        assert_eq!(c, 0);
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
        let you = Player::new(8, 5, 5, "".to_string());
        let boss = Player::new(12, 7, 2, "".to_string());
        assert!(you.is_win(&boss));

        let you = Player::new(8, 5, 5, "".to_string());
        let boss = Player::new(13, 7, 2, "".to_string());
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
        // damage 6 vs 5
        // 104 - 6 * 17 + 2
        // 100 - 5 * 20 + 0
    }

    #[test]
    fn test_attack() {
        let you = Player::new(8, 5, 5, "".to_string());
        let mut boss = Player::new(12, 7, 2, "".to_string());
        you.attack_to(&mut boss);
        assert_eq!(boss.hit_points, 9);
    }

    #[test]
    fn test_attack_defender_has_more_armor() {
        let you = Player::new(8, 5, 5, "".to_string());
        let mut boss = Player::new(12, 7, 200, "".to_string());
        you.attack_to(&mut boss);
        assert_eq!(boss.hit_points, 11);
    }
}