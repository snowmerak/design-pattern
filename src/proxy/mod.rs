use std::collections::HashMap;


pub struct Database {
    pub datas: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            datas: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.datas.insert(key, value);
    }

    pub fn get(&self, key: String) -> String {
        self.datas.get(&key).unwrap().to_string()
    }
}

pub struct DatabaseProxy<'a> {
    datas: Box<&'a mut Database>,
}

impl DatabaseProxy<'_> {
    pub fn new(database: &mut Database) -> DatabaseProxy {
        DatabaseProxy {
            datas: Box::new(database),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.datas.set(key, value)
    }

    pub fn get(&self, key: String) -> String {
        self.datas.get(key)
    }
}

pub fn example() {
    let mut database = Database::new();

    let mut proxy = DatabaseProxy::new(&mut database);

    proxy.set("a".to_string(), "A".to_string());
    proxy.set("b".to_string(), "B".to_string());
    proxy.set("c".to_string(), "C".to_string());

    println!("{}", proxy.get("a".to_string()));
    println!("{}", database.get("b".to_string()));
}