
pub struct Memento {
    state: String,
    versions: Vec<String>,
}

impl Memento {
    pub fn new(init: &str) -> Self {
        Self {
            state: init.to_string(),
            versions: Vec::new(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }

    pub fn set_state(&mut self, state: &str) {
        self.versions.push(self.state.clone());
        self.state = state.to_string();
    }

    pub fn get_version(&self, index: usize) -> Option<String> {
        if index >= self.versions.len() {
            return None;
        }
        Some(self.versions[index].clone())
    }

    pub fn restore(&mut self, index: usize) {
        if index >= self.versions.len() {
            return;
        }
        self.state = self.get_version(index).unwrap();
        self.versions.truncate(index);
    }
}

pub fn example() {
    let mut memento = Memento::new("ground state");
    println!("{}", memento.get_state());

    memento.set_state("first state");
    memento.set_state("second state");
    memento.set_state("third state");
    memento.set_state("fourth state");
    memento.set_state("fifth state");

    println!("{}", memento.get_state());

    memento.restore(1);
    println!("{}", memento.get_state());
}