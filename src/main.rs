#[macro_use]
extern crate rocket;
extern crate dotenv;
use dotenv::dotenv;
use rocket::custom;
use std::env;

use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct APIResponse {
    message: String,
}

#[get("/")]
fn index() -> Json<APIResponse> {
    Json(APIResponse {
        message: "Hello, world! This is rocket".to_string(),
    })
}

#[get("/ping")]
fn ping() -> Json<APIResponse> {
    Json(APIResponse {
        message: "Hello, world! This is ping".to_string(),
    })
}

#[get("/beta")]
fn beta() -> Json<APIResponse> {
    Json(APIResponse {
        message: "Hello, world! This is beta".to_string(),
    })
}

// Configuration struct to hold env variables
struct EnvConfig {
    // rate_limit: u32,
    // redis_url: String,
    // fsk: String,
    port: u16,
}

impl EnvConfig {
    fn from_env() -> EnvConfig {
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

#[rocket::main]
async fn main() {
    dotenv().ok();
    // Load configuration
    let env_config = EnvConfig::from_env();
    // Building Rocket Server Config
    let config = custom(rocket::Config {
        port: env_config.port, // Reading PORT from Env variable
        address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
        ..rocket::Config::debug_default()
    });
    // Building the Server
    let _ = config
        .mount("/", rocket::routes![index])
        .mount("/", rocket::routes![ping])
        .mount("/", rocket::routes![beta])
        .launch()
        .await;
}
