

#[derive(Debug, Deserialize)]
pub struct Config {
    pub auth_token: String,
    pub workers: usize,
    pub redis_address: String,
}
