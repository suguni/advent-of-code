//
/*
struct Boss {
    hit_points: i32,
    damage: i32,
}

const BOSS: Boss = Boss {
    hit_points: 71,
    damage: 10,
};

struct Wizard {
    hit_points: i32,
    armor: i32,
    mana: i32,
    magics: Vec<Magic>,
}

struct Magic {
    cost: i32,
    turns: i32,
}
// enum Magic {
//     MagicMissle,
//     Drain,
//     Shield,
//     Poison,
//     Recharge,
// }

impl Wizard {
    // fn spell(&mut self, magic: &Magic, other: &mut Boss) {
    //     self.mana -= magic.cost();
    //     magics.push(magic.clone());
    // }

    fn turn(&mut self, magic: Magic) {
        // self.effect_spells();
        // self.cast(magic);
    }

    fn effect_spell(&mut self) {}

    fn cast(&mut self, magic: Magic) {}
}

fn player_turn(wizard: &mut Wizard, boss: &mut Boss, magics: &mut Vec<Magic>, magic: Magic) {
    assert!(wizard.mana - magic.cost >= 0);
    // assert!(magics.iter().find(|m| *m == magic).is_none());

    for magic in magics.iter() {
        // magic.effect(wizard, boss);
    }

    magics.retain(|magic| magic.turns > 0);

    wizard.mana -= magic.cost;
    // magic.effect(wizard, boss);
}

fn boss_turn(wizard: Wizard, boss: Boss, magics: Vec<Magic>) {}

#[cfg(test)]
mod tests {

    #[derive(Debug, PartialEq)]
    enum Hello {
        World(i32),
        Bob(i32, i32),
    }

    #[test]
    fn is_ok() {
        let mut h1 = Hello::World(10);
        let mut h2 = Hello::Bob(1, 2);
        let mut h3 = Hello::World(20);

        println!("h1 == h3 {:?}", h1 == h3);

        match h1 {
            Hello::World(ref mut v) => { *v += 10; }
            _ => {}
        }

        match h2 {
            Hello::Bob(ref mut a, ref mut b) => {
                *a += 10;
                *b += 10;
            },
            _ => {}
        }

        println!("{:?}", h1);
        println!("{:?}", h2);
        println!("h1 == h3 {:?}", h1 == h3);
    }
}
*/