use redis::{ConnectionLike, RedisError};
use rocket::serde::json::Json;
use serde::Serialize;

use crate::{
    controller::{create_client, RedisClient},
    env_config::EnvConfig,
};

#[derive(Serialize)]
pub struct APIResponse {
    message: String,
}

#[get("/")]
pub fn index() -> Json<APIResponse> {
    let env: EnvConfig = EnvConfig::from_env();
    let mut rconn = create_client(env.redis_url.as_str()).unwrap();
    if rconn.client.check_connection() {
        println!("The Redis Connection is Established");
    } else {
        println!("Connection is not established,")
    }
    Json(APIResponse {
        message: "Hello, world! This is rocket".to_string(),
    })
}

#[get("/ping")]
pub fn ping() -> Json<APIResponse> {
    Json(APIResponse {
        message: "Hello, world! This is ping".to_string(),
    })
}

#[get("/beta")]
pub fn beta() -> Json<APIResponse> {
    Json(APIResponse {
        message: "Hello, world! This is beta".to_string(),
    })
}
