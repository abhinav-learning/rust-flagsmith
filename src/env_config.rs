use std::env;

// Configuration struct to hold env variables
pub struct EnvConfig {
    // pub rate_limit: u32,
    // pub redis_url: String,
    // pub fsk: String,
    pub port: u16,
}

impl EnvConfig {
    pub fn from_env() -> EnvConfig {
        EnvConfig {
            // rate_limit: env::var("RATE_LIMIT")
            //     .unwrap_or_else(|_| "100".to_string())
            //     .parse()
            //     .unwrap_or(100),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            // redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "localhost:6379".to_string()),
            // fsk: env::var("FLAGSMITH_ENVIRONMENT_KEY")
            //     .unwrap_or_else(|_| "FLAGSMITH_ENVIRONMENT_KEY".to_string()),
        }
    }
}