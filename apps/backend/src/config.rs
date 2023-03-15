pub struct Config {
    pub ip: String,
    pub port: u32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ip: String::from(""),
            port: 0,
        }
    }
}

pub fn read_from_envfile() {}
