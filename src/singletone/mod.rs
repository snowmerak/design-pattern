use std::sync::Mutex;

struct Singletone {
    address: String,
    port: u16,
}

static mut SINGLETONE: Option<Mutex<Singletone>> = None;

impl Singletone {
    pub fn get() -> &'static Mutex<Singletone> {
        unsafe {
            if SINGLETONE.is_none() {
                SINGLETONE = Some(Mutex::new(Singletone {
                    address: String::from("localhost"),
                    port: 8080,
                }));
            }
            SINGLETONE.as_ref().unwrap()
        }
    }

    pub fn set_address(address: String) {
        unsafe {
            match SINGLETONE {
                Some(ref mut singleton) => {
                    let mut singleton = singleton.lock().unwrap();
                    singleton.address = address;
                }
                None => {
                    panic!("Singleton is not initialized");
                }
            }
        }
    }

    pub fn set_port(port: u16) {
        unsafe {
            match SINGLETONE {
                Some(ref mut singleton) => {
                    let mut singleton = singleton.lock().unwrap();
                    singleton.port = port;
                }
                None => {
                    panic!("Singleton is not initialized");
                }
            }
        }
    }
}

pub fn example() {
    {
        let singleton = Singletone::get();
        let sg = singleton.lock().unwrap();
        println!("{}:{}", sg.address, sg.port);
    }
    {
        Singletone::set_address("127.0.0.1".to_string());
        Singletone::set_port(3000);
        
        let singleton = Singletone::get();
        let sg = singleton.lock().unwrap();
        println!("{}:{}", sg.address, sg.port);
    }
    {
        let singleton = Singletone::get();
        let sg = singleton.lock().unwrap();
        println!("{}:{}", sg.address, sg.port);
    }
}
