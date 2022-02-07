
pub struct Mechanic {
    pub name: String,
    pub hp: i16,
    pub armor: i16,
    pub attack: i16,
}

pub fn train(name: &str) -> Result<Mechanic, String> {
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