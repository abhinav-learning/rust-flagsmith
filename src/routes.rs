use crate::controller::get_redis_client;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse {
    message: String,
}

#[get("/")]
pub fn index() -> Json<APIResponse> {
    let con = get_redis_client();
    println!("Connection value is {}", con);
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
