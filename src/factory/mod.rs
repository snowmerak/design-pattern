
pub struct Mechanic {
    name: String,
    hp: i16,
    armor: i16,
    attack: i16,
}

pub trait Terran {
    fn get_name(&self) -> String;
    fn get_hp(&self) -> i16;
    fn get_armor(&self) -> i16;
    fn get_attack(&self) -> i16;
    fn decrease_hp(&mut self, attack: i16);
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

const VULTURE: &str = "vulture";
const TANK: &str = "tank";
const GOLIATH: &str = "goliath";

pub fn train(name: &str) -> Result<impl Terran, String> {
    match name {
        "vulture" => Ok(Mechanic {
            name: "Vulture".to_string(),
            hp: 75,
            armor: 0,
            attack: 20,
        }),
        "tank" => Ok(Mechanic {
            name: "Tank".to_string(),
            hp: 150,
            armor: 20,
            attack: 20,
        }),
        "goliath" => Ok(Mechanic {
            name: "Goliath".to_string(),
            hp: 125,
            armor: 10,
            attack: 20,
        }),
        _ => Err(String::from("Invalid name")),
    }
}

pub fn example() {
    let mut vulture = train(VULTURE).unwrap();
    let mut tank = train(TANK).unwrap();
    let mut goliath = train(GOLIATH).unwrap();

    tank.decrease_hp(vulture.get_attack());
    goliath.decrease_hp(tank.get_attack());
    vulture.decrease_hp(goliath.get_attack());

    println!("{}: {}", vulture.get_name(), vulture.get_hp());
    println!("{}: {}", tank.get_name(), tank.get_hp());
    println!("{}: {}", goliath.get_name(), goliath.get_hp());
}