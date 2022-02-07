
#[derive(Debug)]
pub struct Marine {
    pub name: String,
    pub attack: i16,
    pub armor: i16,
    pub hp: i16,
}

pub struct MarineBuilder {
    name: String,
    attack: i16,
    armor: i16,
    hp: i16,
}

impl MarineBuilder {
    pub fn new() -> MarineBuilder {
        MarineBuilder {
            name: String::from("Marine"),
            attack: 0,
            armor: 0,
            hp: 0,
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut MarineBuilder {
        self.name = name;
        self
    }

    pub fn set_attack(&mut self, attack: i16) -> &mut MarineBuilder {
        self.attack = attack;
        self
    }

    pub fn set_armor(&mut self, armor: i16) -> &mut MarineBuilder {
        self.armor = armor;
        self
    }

    pub fn set_hp(&mut self, hp: i16) -> &mut MarineBuilder {
        self.hp = hp;
        self
    }

    pub fn build(&self) -> Marine {
        Marine {
            name: self.name.clone(),
            attack: self.attack,
            armor: self.armor,
            hp: self.hp,
        }
    }
}

pub fn example() {
    let marine = MarineBuilder::new()
        .set_name("Marine".to_string())
        .set_attack(20)
        .set_armor(5)
        .set_hp(40)
        .build();
    println!("{:?}", marine);
}