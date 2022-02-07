
pub trait Bootcamp {
    fn train(&self, name: &str) -> Result<Box<dyn Terran>, String>;
}

pub trait Terran {
    fn get_name(&self) -> String;
    fn get_hp(&self) -> i16;
    fn get_armor(&self) -> i16;
    fn get_attack(&self) -> i16;
    fn decrease_hp(&mut self, attack: i16);
}

pub struct Bionic {
    name: String,
    hp: i16,
    armor: i16,
    attack: i16,
}

impl Terran for Bionic {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_hp(&self) -> i16 {
        self.hp
    }

    fn get_armor(&self) -> i16 {
        self.armor
    }

    fn get_attack(&self) -> i16 {
        self.attack
    }

    fn decrease_hp(&mut self, attack: i16) {
        self.hp -= attack-self.armor;
    }
}

pub struct Barrack {}

impl Bootcamp for Barrack {
    fn train(&self, name: &str) -> Result<Box<dyn Terran>, String> {
        match name {
            "marine" => Ok(Box::new(Bionic {
                name: "marine".to_string(),
                hp: 40,
                armor: 0,
                attack: 10,
            })),
            "firebat" => Ok(Box::new(Bionic {
                name: "firebat".to_string(),
                hp: 60,
                armor: 0,
                attack: 6,
            })),
            "medic" => Ok(Box::new(Bionic {
                name: "medic".to_string(),
                hp: 80,
                armor: 0,
                attack: 0,
            })),
            "ghost" => Ok(Box::new(Bionic {
                name: "ghost".to_string(),
                hp: 40,
                armor: 0,
                attack: 20,
            })),
            _ => Err("Unknown unit".to_string()),
        }
    }
}

pub struct Mechanic {
    name: String,
    hp: i16,
    armor: i16,
    attack: i16,
}

impl Terran for Mechanic {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_hp(&self) -> i16 {
        self.hp
    }

    fn get_armor(&self) -> i16 {
        self.armor
    }

    fn get_attack(&self) -> i16 {
        self.attack
    }

    fn decrease_hp(&mut self, attack: i16) {
        self.hp -= attack-self.armor;
    }
}

pub struct Factory {}

impl Bootcamp for Factory {
    fn train(&self, name: &str) -> Result<Box<dyn Terran>, String> {
        match name {
            "vulture" => Ok(Box::new(Mechanic {
                name: "marauder".to_string(),
                hp: 75,
                armor: 0,
                attack: 10,
            })),
            "goliath" => Ok(Box::new(Mechanic {
                name: "hellion".to_string(),
                hp: 125,
                armor: 0,
                attack: 15,
            })),
            "tank" => Ok(Box::new(Mechanic {
                name: "tank".to_string(),
                hp: 200,
                armor: 0,
                attack: 20,
            })),
            _ => Err("Unknown unit".to_string()),
        }
    }
}

pub fn build_bootcamp(name: &str) -> Result<Box<dyn Bootcamp>, String> {
    match name {
        "barracks" => Ok(Box::new(Barrack {})),
        "factory" => Ok(Box::new(Factory {})),
        _ => Err("Unknown bootcamp".to_string()),
    }
}

pub fn example() {
    let barrack = build_bootcamp("barracks").unwrap();
    let mut marine = barrack.train("marine").unwrap();
    println!("{}", marine.get_name());

    let factory = build_bootcamp("factory").unwrap();
    let mut vulture = factory.train("vulture").unwrap();
    println!("{}", vulture.get_name());

    vulture.decrease_hp(marine.get_attack());
    println!("{}", vulture.get_hp());

    marine.decrease_hp(vulture.get_attack());
    println!("{}", marine.get_hp());
}