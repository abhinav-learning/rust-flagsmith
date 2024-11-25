#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::custom;

mod env_config;
mod routes;

#[rocket::main]
async fn main() {
    dotenv().ok();
    // Load configuration
    let env_config = env_config::EnvConfig::from_env();
    // Building Rocket Server Config
    let config = custom(rocket::Config {
        port: env_config.port, // Reading PORT from Env variable
        address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
        ..rocket::Config::debug_default()
    });
    // Building the Server
    let _ = config
        .mount("/", rocket::routes![routes::index])
        .mount("/", rocket::routes![routes::ping])
        .mount("/", rocket::routes![routes::beta])
        .launch()
        .await;
}
