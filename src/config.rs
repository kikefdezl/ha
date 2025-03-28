use std::env;

#[derive(Clone)]
pub struct Config {
    pub host_url: String,
    pub linak_service_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host_url: env::var("HOST_URL").expect("HOST_URL not set"),
            linak_service_url: env::var("LINAK_SERVICE_URL").expect("LINAK_SERVICE_URL not set"),
        }
    }
}
