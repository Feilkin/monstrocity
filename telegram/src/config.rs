
#[derive(Debug, Deserialize)]
pub struct Webhook {
    pub bind_address: String,
    pub external_address: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub auth_token: String,
    pub webhook: Webhook,
}
