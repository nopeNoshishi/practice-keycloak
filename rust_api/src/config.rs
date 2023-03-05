#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub keycloak_url: String,
    pub keycloak_auth_url: String,
    pub keycloak_token_url: String,
}

impl Config {
    pub fn init() -> Config {
        let app_url = std::env::var("APP_URL").expect("APP_URL must be set");
        let keycloak_url = std::env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
        let keycloak_auth_url = std::env::var("KEYCLOAK_AUTH_URL").expect("KEYCLOAK_AUTH_URL must be set");
        let keycloak_token_url = std::env::var("KEYCLOAK_TOKEN_URL").expect("KEYCLOAK_TOKEN_URL must be set");
        Config {
            app_url,
            keycloak_url,
            keycloak_auth_url,
            keycloak_token_url,
        }
    }
}