use crate::config::Config;

pub struct ConfigService {
    pub config: Config,
}

impl ConfigService {
    pub fn new() -> Self {
        ConfigService {
            config: Config::new(),
        }
    }
}
