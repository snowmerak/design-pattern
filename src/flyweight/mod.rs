use std::collections::HashMap;

pub struct FigureFactory {
    figures: HashMap<String, Figure>,
}

impl FigureFactory {
    pub fn new() -> FigureFactory {
        FigureFactory {
            figures: HashMap::new(),
        }
    }

    pub fn add<'a>(&mut self, name: &'a str, height: i16, width: i16) -> bool {
        if !self.figures.contains_key(name) {
            let figure = Figure::new(name, height, width);
            self.figures.insert(name.to_string(), figure);
            return true;
        }
        false
    }

    pub fn get(&self, name: &str) -> Option<&Figure> {
        self.figures.get(name)
    }
}

pub struct Figure {
    pub name: String,
    pub height: i16,
    pub width: i16,
}

impl Figure {
    pub fn new(name: &str, height: i16, width: i16) -> Figure {
        Figure {
            name: name.to_string(),
            height,
            width,
        }
    }

    pub fn draw(&self) {
        println!("{}: {} * {}", self.name, self.height, self.width);
    }
}

pub fn example() {
    let mut figure_factory = FigureFactory::new();
    println!("{} {} {}", figure_factory.add("figure1", 10, 20), figure_factory.add("figure2", 10, 20), figure_factory.add("figure1", 20, 40));
    figure_factory.get("figure1").unwrap().draw();
    figure_factory.get("figure2").unwrap().draw();
}