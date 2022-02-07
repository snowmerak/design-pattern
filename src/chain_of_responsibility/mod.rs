
pub trait CompanyMan {
    fn make_signature(&self);
    fn set_next_man(&mut self, next_man: Box<dyn CompanyMan>);
}

pub struct ChairMan {
    pub name: String,
    pub next_man: Option<Box<dyn CompanyMan>>
}

impl ChairMan {
    pub fn new(name: String) -> Self {
        Self {
            name,
            next_man: None,
        }
    }
}

impl CompanyMan for ChairMan {
    fn make_signature(&self) {
        println!("chairman, {} made signature.", self.name);
        match &self.next_man {
            Some(next) => next.make_signature(),
            None => println!("all done!"),
        }
    }

    fn set_next_man(&mut self, next_man: Box<dyn CompanyMan>) {
        self.next_man = Some(next_man);
    }
}

pub struct President {
    pub name: String,
    pub next_man: Option<Box<dyn CompanyMan>>
}

impl President {
    pub fn new(name: String) -> Self {
        Self {
            name,
            next_man: None,
        }
    }
}

impl CompanyMan for President {
    fn make_signature(&self) {
        println!("president, {} made signature.", self.name);
        match &self.next_man {
            Some(next) => next.make_signature(),
            None => println!("all done!"),
        }
    }

    fn set_next_man(&mut self, next_man: Box<dyn CompanyMan>) {
        self.next_man = Some(next_man);
    }
}

pub struct Director {
    pub name: String,
    pub next_man: Option<Box<dyn CompanyMan>>
}

impl Director {
    pub fn new(name: String) -> Self {
        Self {
            name,
            next_man: None,
        }
    }
}

impl CompanyMan for Director {
    fn make_signature(&self) {
        println!("director, {} made signature.", self.name);
        match &self.next_man {
            Some(next) => next.make_signature(),
            None => println!("all done!"),
        }
    }

    fn set_next_man(&mut self, next_man: Box<dyn CompanyMan>) {
        self.next_man = Some(next_man);
    }
}

pub struct Manager {
    pub name: String,
    pub next_man: Option<Box<dyn CompanyMan>>
}

impl Manager {
    pub fn new(name: String) -> Self {
        Self {
            name,
            next_man: None,
        }
    }
}

impl CompanyMan for Manager {
    fn make_signature(&self) {
        println!("manager, {} made signature.", self.name);
        match &self.next_man {
            Some(next) => next.make_signature(),
            None => println!("all done!"),
        }
    }

    fn set_next_man(&mut self, next_man: Box<dyn CompanyMan>) {
        self.next_man = Some(next_man);
    }
}

pub struct Staff {
    pub name: String,
    pub next_man: Option<Box<dyn CompanyMan>>
}

impl Staff {
    pub fn new(name: String) -> Self {
        Self {
            name,
            next_man: None,
        }
    }
}

impl CompanyMan for Staff {
    fn make_signature(&self) {
        println!("staff, {} made signature.", self.name);
        match &self.next_man {
            Some(next) => next.make_signature(),
            None => println!("all done!"),
        }
    }

    fn set_next_man(&mut self, next_man: Box<dyn CompanyMan>) {
        self.next_man = Some(next_man);
    }
}

pub fn example() {
    let chairman = ChairMan::new("Jung".to_string());
    let mut president = President::new("Lee".to_string());
    let mut director = Director::new("kim".to_string());
    let mut manager = Manager::new("park".to_string());
    let mut staff = Staff::new("choi".to_string());

    president.set_next_man(Box::new(chairman));
    director.set_next_man(Box::new(president));
    manager.set_next_man(Box::new(director));
    staff.set_next_man(Box::new(manager));

    staff.make_signature();
}