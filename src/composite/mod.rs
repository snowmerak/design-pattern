
pub trait Countable {
    fn count(&self, id: &str) -> i64;
}
    
pub struct OneLineString {
    value: String,
}

impl OneLineString {
    pub fn new(value: String) -> Box<OneLineString> {
        Box::new(OneLineString {
            value,
        })
    }
}

impl Countable for OneLineString {
    fn count(&self, id: &str) -> i64 {
        self.value.matches(id).count() as i64
    }
}

pub struct MultiLineString {
    value: Vec<Box<dyn Countable>>,
}

impl MultiLineString {
    pub fn new() -> Self {
        MultiLineString {
            value: Vec::new(),
        }
    }

    pub fn add(&mut self, line: Box<dyn Countable>) {
        self.value.push(line);
    }
}

impl Countable for MultiLineString {
    fn count(&self, id: &str) -> i64 {
        self.value.iter().map(|line| (*line).count(id)).sum()
    }
}

pub fn example() {
    let mut s = MultiLineString::new();
    s.add(OneLineString::new("What will be, will be. ".to_string()));
    s.add(OneLineString::new("When I look back on my life, ".to_string()));
    s.add(OneLineString::new("I will be able to tell you ".to_string()));
    s.add(OneLineString::new("That I've been a good".to_string()));

    let mut next_multi_line_string = MultiLineString::new();
    next_multi_line_string.add(OneLineString::new("What will be, will be. ".to_string()));
    next_multi_line_string.add(OneLineString::new("When I look back on my life, ".to_string()));
    next_multi_line_string.add(OneLineString::new("I will be able to tell you ".to_string()));
    next_multi_line_string.add(OneLineString::new("That I've been a good".to_string()));

    s.add(Box::new(next_multi_line_string));

    s.add(OneLineString::new("I will be able to tell you ".to_string()));
    s.add(OneLineString::new("That I've been a good".to_string()));

    let will_count = s.count("will");
    let be_count = s.count("be");
    let able_count = s.count("able");

    println!("will_count: {}", will_count);
    println!("be_count: {}", be_count);
    println!("able_count: {}", able_count);
}