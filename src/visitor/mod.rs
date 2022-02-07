
#[derive(Debug)]
pub struct Marine {
    pub hp: i16,
    pub armor: i16,
    pub weapon: i16,
}

#[derive(Debug)]
pub struct Hydra {
    pub hp: i16,
    pub armor: i16,
    pub weapon: i16,
    pub hp_gen_rate: f32,
}

pub trait Visitor {
    fn visit_marine(&self, marine: &mut Marine);
    fn visit_hydra(&self, hydra: &mut Hydra);
}

pub struct VisitorImpl;

impl VisitorImpl {
    pub fn new() -> Self {
        VisitorImpl {}
    }
}

impl Visitor for VisitorImpl {
    fn visit_marine(&self, marine: &mut Marine) {
        marine.hp += 1;
        marine.armor += 1;
        marine.weapon += 1;
    }

    fn visit_hydra(&self, hydra: &mut Hydra) {
        hydra.hp += 1;
        hydra.armor += 1;
        hydra.weapon += 1;
        hydra.hp_gen_rate += 0.1;
    }
}

pub fn example() {
    let mut marine = Marine {
        hp: 100,
        armor: 100,
        weapon: 100,
    };
    let mut hydra = Hydra {
        hp: 100,
        armor: 100,
        weapon: 100,
        hp_gen_rate: 0.1,
    };

    let visitor = VisitorImpl::new();
    visitor.visit_marine(&mut marine);
    visitor.visit_hydra(&mut hydra);

    println!("marine: {:?}", marine);
    println!("hydra: {:?}", hydra);
}
