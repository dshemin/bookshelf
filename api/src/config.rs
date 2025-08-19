use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub auth: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

pub fn load() -> Config {
    Config {
        auth: AuthConfig {
            auth_url: env::var("BOOKSHELF_AUTH_URL").unwrap_or(String::new()),
            token_url: env::var("BOOKSHELF_TOKEN_URL").unwrap_or(String::new()),
            client_id: env::var("BOOKSHELF_CLIENT_ID").unwrap_or(String::new()),
            client_secret: env::var("BOOKSHELF_CLIENT_SECRET").unwrap_or(String::new()),
            redirect_url: env::var("BOOKSHELF_REDIRECT_URL").unwrap_or(String::new()),
        },
    }
}
