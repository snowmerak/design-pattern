
pub trait Hamburger {
    fn name(&self) -> String;
    fn price(&self) -> i16;
}

pub struct CheeseBurger {
    name: String,
    price: i16,
}

impl CheeseBurger {
    fn new() -> Box<dyn Hamburger> {
        Box::new(CheeseBurger {
            name: "Cheese Burger".to_string(),
            price: 10,
        })
    }
}

impl Hamburger for CheeseBurger {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn price(&self) -> i16 {
        self.price
    }
}

pub struct CheeseAddedBurger {
    hambuger: Box<dyn Hamburger>,
}

impl CheeseAddedBurger {
    fn new(hambuger: Box<dyn Hamburger>) -> Box<dyn Hamburger> {
        Box::new(CheeseAddedBurger {
            hambuger,
        })
    }
}

impl Hamburger for CheeseAddedBurger {
    fn name(&self) -> String {
        format!("{} + Cheese", self.hambuger.name())
    }

    fn price(&self) -> i16 {
        self.hambuger.price() + 2
    }
}

pub struct TomatoAddedBurger {
    hambuger: Box<dyn Hamburger>,
}

impl TomatoAddedBurger {
    fn new(hambuger: Box<dyn Hamburger>) -> Box<dyn Hamburger> {
        Box::new(TomatoAddedBurger {
            hambuger,
        })
    }
}

impl Hamburger for TomatoAddedBurger {
    fn name(&self) -> String {
        format!("{} + Tomato", self.hambuger.name())
    }

    fn price(&self) -> i16 {
        self.hambuger.price() + 1
    }
}
    
pub struct BeefAddedBurger {
    hambuger: Box<dyn Hamburger>,
}

impl BeefAddedBurger {
    fn new(hambuger: Box<dyn Hamburger>) -> Box<dyn Hamburger> {
        Box::new(BeefAddedBurger {
            hambuger,
        })
    }
}

impl Hamburger for BeefAddedBurger {
    fn name(&self) -> String {
        format!("{} + Beef", self.hambuger.name())
    }

    fn price(&self) -> i16 {
        self.hambuger.price() + 3
    }
}

pub struct LettuesAddedBurger {
    hambuger: Box<dyn Hamburger>,
}

impl LettuesAddedBurger {
    fn new(hambuger: Box<dyn Hamburger>) -> Box<dyn Hamburger> {
        Box::new(LettuesAddedBurger {
            hambuger,
        })
    }
}

impl Hamburger for LettuesAddedBurger {
    fn name(&self) -> String {
        format!("{} + Lettues", self.hambuger.name())
    }

    fn price(&self) -> i16 {
        self.hambuger.price() + 1
    }
}
    
pub fn example() {
    let burger = CheeseBurger::new();
    let cheese_burger = CheeseAddedBurger::new(burger);
    let tomato_burger = TomatoAddedBurger::new(cheese_burger);
    let beef_burger = BeefAddedBurger::new(tomato_burger);
    let lettues_burger = LettuesAddedBurger::new(beef_burger);

    println!("{}", lettues_burger.name());
    println!("{}", lettues_burger.price());
}