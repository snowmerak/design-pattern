
pub trait Command {
    fn excute(&self);
}

pub struct SaveCommand {
    name: String,
}

impl SaveCommand {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Command for SaveCommand {
    fn excute(&self) {
        println!("{} save command excuted!", self.name);
    }
}

pub struct ExitCommand {
    name: String,
}

impl ExitCommand {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Command for ExitCommand {
    fn excute(&self) {
        println!("{} exit command excuted!", self.name);
    }
}

pub struct OpenCommand {
    name: String,
}

impl OpenCommand {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Command for OpenCommand {
    fn excute(&self) {
        println!("{} open command excuted!", self.name);
    }
}

pub struct Button<'a> {
    command: Box<&'a dyn Command>,
}

impl<'a> Button<'a> {
    pub fn new(command: Box<&'a dyn Command>) -> Self {
        Self {
            command,
        }
    }

    pub fn press(&self) {
        self.command.excute();
    }
}

pub fn example() {
    let save_command = SaveCommand::new("app");
    let exit_command = ExitCommand::new("app");
    let open_command = OpenCommand::new("app");

    let save_button = Button::new(Box::new(&save_command));
    let exit_button = Button::new(Box::new(&exit_command));
    let open_button = Button::new(Box::new(&open_command));

    save_button.press();
    exit_button.press();
    open_button.press();

    let save_touch = Button::new(Box::new(&save_command));
    let exit_touch = Button::new(Box::new(&exit_command));
    let open_touch = Button::new(Box::new(&open_command));

    save_touch.press();
    exit_touch.press();
    open_touch.press();
}