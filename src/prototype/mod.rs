
#[derive(Debug)]
pub struct Marine {
    pub name: String,
    pub attack: i16,
    pub armor: i16,
    pub hp: i16,
}

pub trait Cloneable {
    fn clone(&self) -> Self;
}

impl Cloneable for Marine {
    fn clone(&self) -> Self {
        Marine {
            name: self.name.clone(),
            attack: self.attack,
            armor: self.armor,
            hp: self.hp,
        }
    }
}

pub fn example() {
    let marine_a = Marine {
        name: String::from("Marine"),
        attack: 10,
        armor: 10,
        hp: 10,
    };

    let mut marine_b = marine_a.clone();
    marine_b.name = String::from("jim");

    println!("marine_a: {:?}", marine_a);
    println!("marine_b: {:?}", marine_b);
}