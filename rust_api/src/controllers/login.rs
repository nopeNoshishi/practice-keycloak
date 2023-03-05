use actix_web::{
    HttpResponse, Responder,
    http::header};
// use std::env;

use log;


pub async fn login() -> impl Responder {
    log::info!("Login API");

    let auth_url = "http://localhost:4000/auth/realms/test/protocol/openid-connect/auth"; // env::var("KEYCLOAK_AUTH_URL").expect("環境変数がありません");
    let client_id = "test-app"; // env::var("CLIENT_ID").expect("環境変数がありません");

    let login_url = auth_url.to_string()
        + "?response_type=code"
        + "&client_id=" + client_id
        + "&redirect_uri=" + "http://localhost:8080/getcode"
        + "&scope=openid";

    log::info!("{}", login_url);

    HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, login_url)
        .finish()
}
