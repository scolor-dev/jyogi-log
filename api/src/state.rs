use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
}

impl AppState {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }
}
