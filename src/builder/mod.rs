
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

pub trait BuilderTrait {
    fn new() -> MarineBuilder;
    fn set_name(&mut self, name: String) -> &mut MarineBuilder;
    fn set_attack(&mut self, attack: i16) -> &mut MarineBuilder;
    fn set_armor(&mut self, armor: i16) -> &mut MarineBuilder;
    fn set_hp(&mut self, hp: i16) -> &mut MarineBuilder;
    fn build(&self) -> Marine;
}

impl BuilderTrait for MarineBuilder {
    fn new() -> MarineBuilder {
        MarineBuilder {
            name: String::from(""),
            attack: 0,
            armor: 0,
            hp: 0,
        }
    }

    fn set_name(&mut self, name: String) -> &mut MarineBuilder {
        self.name = name;
        self
    }

    fn set_attack(&mut self, attack: i16) -> &mut MarineBuilder {
        self.attack = attack;
        self
    }

    fn set_armor(&mut self, armor: i16) -> &mut MarineBuilder {
        self.armor = armor;
        self
    }

    fn set_hp(&mut self, hp: i16) -> &mut MarineBuilder {
        self.hp = hp;
        self
    }

    fn build(&self) -> Marine {
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