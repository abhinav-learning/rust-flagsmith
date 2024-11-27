use redis::{Client, Commands};

pub trait RedisClientImpl {
    fn connect(&self) -> Result<redis::Connection, redis::RedisError>;
    fn set(&mut self, key: String, value: String) -> Result<(), redis::RedisError>;
    fn get(&mut self, key: String) -> Result<Option<String>, redis::RedisError>;
}

pub struct RedisClient {
    pub client: Client,
}

impl RedisClientImpl for RedisClient {
    fn connect(&self) -> Result<redis::Connection, redis::RedisError> {
        self.client.get_connection()
    }

    fn set(&mut self, key: String, value: String) -> Result<(), redis::RedisError> {
        let mut con = self.connect()?;
        con.set(key, value)
    }

    fn get(&mut self, key: String) -> Result<Option<String>, redis::RedisError> {
        let mut con = self.connect()?;
        con.get(key)
    }
}

pub fn create_client(redis_url: &str) -> Result<RedisClient, redis::RedisError> {
    Ok(RedisClient {
        client: Client::open(redis_url)?,
    })
}
