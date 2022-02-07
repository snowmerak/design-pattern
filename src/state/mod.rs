
pub struct Laptop {
    pub state: bool,
}

impl Laptop {
    pub fn new() -> Self {
        Self { state: false}
    }
    
}

pub trait OnOffState {
    fn on(&mut self);
    fn off(&mut self);
}

impl OnOffState for Laptop {
    fn on(&mut self) {
        self.state = true;
    }

    fn off(&mut self) {
        self.state = false;
    }
}

pub fn example() {
    let mut laptop = Laptop::new();
    println!("laptop: {}", laptop.state);
    laptop.on();
    println!("laptop: {}", laptop.state);
    laptop.off();
    println!("laptop: {}", laptop.state);
}