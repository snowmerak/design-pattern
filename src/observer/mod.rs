
pub struct Observer {}

impl Observer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, msg: String) {
        println!("update: {}", msg);
    }
}

pub struct Notifier<'a> {
    pub observers: Vec<&'a Observer>,
}

impl<'a> Notifier<'a> {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }

    pub fn add_observer(&mut self, observer: &'a Observer) {
        self.observers.push(observer);
    }

    pub fn notify(&mut self, msg: String) {
        for observer in &self.observers {
            (*observer).update(msg.clone());
        }
    }
}

pub fn example() {
    let mut notifier = Notifier::new();

    let observer1 = Observer::new();
    let observer2 = Observer::new();

    notifier.add_observer(&observer1);
    notifier.add_observer(&observer2);

    notifier.notify(String::from("hello"));
    notifier.notify(String::from("world"));
    notifier.notify(String::from("!"));
}
