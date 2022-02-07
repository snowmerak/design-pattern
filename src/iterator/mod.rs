
pub struct User {
    pub name: String,
    pub age: u8,
}

pub struct UserList {
    pub list: Vec<User>,
}

impl UserList {
    pub fn new() -> UserList {
        UserList {
            list: Vec::new(),
        }
    }

    pub fn add(&mut self, user: User) {
        self.list.push(user);
    }

    pub fn get(&self, index: usize) -> Option<&User> {
        self.list.get(index)
    }

    pub fn iter(&self) -> UserIterator {
        UserIterator {
            list: self.list.iter().collect(),
            index: 0,
        }
    }
}

pub struct UserIterator<'a> {
    pub list: Vec<&'a User>,
    pub index: usize,
}

impl UserIterator<'_> {
    pub fn next(&mut self) -> Option<&User> {
        if self.index >= self.list.len() {
            return None;
        }
        let result = self.list.get(self.index);
        self.index += 1;
        Some(result.unwrap())
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

pub fn example() {
    let mut user_list = UserList::new();
    user_list.add(User {
        name: "Alice".to_string(),
        age: 20,
    });
    user_list.add(User {
        name: "Bob".to_string(),
        age: 30,
    });
    user_list.add(User {
        name: "Charlie".to_string(),
        age: 40,
    });

    let mut iter = user_list.iter();
    while let Some(user) = iter.next() {
        println!("{}", user.name);
    }
    iter.reset();
    while let Some(user) = iter.next() {
        println!("{}", user.name);
    }
}