#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! This is rocket"
}

#[get("/ping")]
fn ping() -> &'static str {
    "Hello, world! This is ping"
}

#[get("/beta")]
fn beta() -> &'static str {
    "Hello, world! This is beta"
}

// Configuration struct to hold env variables
struct Config {
    rate_limit: u32,
    app_name: String,
    environment: String,
}


impl Config {
    fn from_env() -> Config {
        Config {
            rate_limit: env::var("RATE_LIMIT")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            app_name: env::var("APP_NAME")
                .unwrap_or_else(|_| "DefaultApp".to_string()),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    // Load configuration
    let config = Config::from_env();
    println!("Starting {} in {} environment", 
             config.app_name, 
             config.environment);
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![ping])
        .mount("/", routes![beta])
}
