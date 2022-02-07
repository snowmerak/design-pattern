
pub struct StringMutator {
    value: String,
    mutator: Option<fn(String) -> String>
}

impl StringMutator {
    pub fn new(value: String) -> StringMutator {
        StringMutator {
            value,
            mutator: None,
        }
    }

    pub fn set_mutator(&mut self, mutator: fn(String) -> String) {
        self.mutator = Some(mutator);
    }

    pub fn mutate(&self) -> String {
        match self.mutator {
            Some(mutator) => mutator(self.value.clone()),
            None => self.value.clone(),
        }
    }
}

pub fn example() {
    let mut mutator = StringMutator::new("Hello".to_string());
    
    mutator.set_mutator(|s| s.to_uppercase());
    println!("{}", mutator.mutate());

    mutator.set_mutator(|s| s.to_lowercase());
    println!("{}", mutator.mutate());

    mutator.set_mutator(|s| s.replace("Hello", "World"));
    println!("{}", mutator.mutate());
}