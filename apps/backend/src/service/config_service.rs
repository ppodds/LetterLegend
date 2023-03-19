use crate::config::Config;

pub struct ConfigService {
    pub config: Config,
}

impl ConfigService {
    pub fn new(config: Config) -> Self {
        ConfigService { config }
    }
}
