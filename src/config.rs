use std::env;

#[derive(Clone)]
pub struct Config {
    pub linak_service_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            linak_service_url: env::var("LINAK_SERVICE_URL").unwrap(),
        }
    }
}
