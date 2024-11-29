use crate::controller::RedisClientImpl;
use crate::utils::get_beta;
use crate::{controller::create_client, env_config::EnvConfig, utils::get_configured_rate_limit};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Response;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse {
    message: String,
}

#[get("/")]
pub fn index() -> Json<APIResponse> {
    Json(APIResponse {
        message: "Hello, world! This is rocket".to_string(),
    })
}

#[get("/ping")]
pub fn ping(req: Request) -> Json<APIResponse> {
    let env: EnvConfig = EnvConfig::from_env();
    let mut rconn = create_client(env.redis_url.as_str()).unwrap();
    if rconn.client.is_open() {
        info!("The Redis Connection is Established");
    } else {
        error!("Connection is not established,")
    }
    let allowed_limit = get_configured_rate_limit();
    let client_ip = req.client_ip().unwrap();
    let current_usage = rconn.get(client_ip.to_string()).unwrap();
    if current_usage.is_none() {
        rconn.set(client_ip.to_string(), allowed_limit.to_string());
        return Json(APIResponse {
            message: "Hello, world! This is Ping Endpoint".to_string(),
        });
    } else {
        let current_usage_number: i64 = current_usage
            .unwrap()
            .parse()
            .expect("Failed to parse integer");
        let pending_usage = allowed_limit - current_usage_number;
        if pending_usage <= 0 {
            return Json(APIResponse {
                message: "You have hit the Rate Limit for the API. Try again later".to_string(),
            });
        } else {
            let data = format!("remaining request for {} is {}", client_ip, pending_usage);
            return Json(APIResponse {
                message: data.to_string(),
            });
        }
    }
}

#[get("/beta")]
pub fn beta() -> Result<Json<APIResponse>, Status> {
    let is_beta = get_beta();
    match is_beta {
        Ok(is_beta) => Json(APIResponse {
            message: "Hello, world! This is beta".to_string(),
        }),
        Err((status, err)) => Err(status),
    }
}
