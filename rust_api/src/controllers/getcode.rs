use actix_web::{web, HttpResponse, Responder, http::header};
use serde::{Serialize, Deserialize};
use reqwest;
use std::collections::HashMap;
use log;

use jsonwebtokens::{raw::TokenSlices, raw};

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    session_state: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    refresh_token: String,
    id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    preferred_username: String,
    iat: i64,
    exp: i64,
}

pub async fn get_code(data: web::Query<Code>) -> impl Responder {
    log::info!("Token API");

    log::info!("code: {:?}", data.code);

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", &data.code);
    params.insert("client_id", "test-app");
    params.insert("client_secret", "ztcq9Fz9h19c03AVG8hiXwkzUYIjpyZW");
    params.insert("redirect_uri", "http://localhost:8080/getcode");

    let client = reqwest::Client::new();
    
    let response = client.post("http://localhost:4000/auth/realms/test/protocol/openid-connect/token")
        .form(&params)
        .send()
        .await.expect("Not working");

    let body = response.text().await.expect("test");

    let content = serde_json::from_str::<Token>(&body).unwrap();

    let secret = "MIIClzCCAX8CBgGGrO4/QjANBgkqhkiG9w0BAQsFADAPMQ0wCwYDVQQDDAR0ZXN0MB4XDTIzMDMwNDE0MDA0MFoXDTMzMDMwNDE0MDIyMFowDzENMAsGA1UEAwwEdGVzdDCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKQDD3ZKuTBZJYTFOo3H+WMHMBnY+8BZpk1VMCAiIpt/UCNIpVkyiGMPOjeeAWM29oB7o0+0Asm/7Z/zskoE0wOvYk1i2CIZy8h8bAbP4ylWrEo1pEFYhJlaNdeu+gBgntP0td3E2Z8uYFREf486X1cwwrZhWj0nCor2NU6m3j2m/Y2zs4Oaw/wreGZsagJVTPhQz6OYyNDnlWpyJ+yO/Lv12fXNM9ZiLODWQPc/7R4KbAL0GMrvkOEnMYmXXBueCbCPKqqIYnxLY4Zs2p923BPW+D3LlWnAU2QklruE0g0kPE7H6Be94BiMr9E419C0SA6T55hqtsoi8LtAVpNxgF8CAwEAATANBgkqhkiG9w0BAQsFAAOCAQEAFuh3x0XP+MQuimZtUfpFZgsiXZKGJ8NVjfMWoHBM9R7NJILmDxDes4kss2REV95i7aXKCqYZmaDMSaupokr7Db3xrZMWt1b5q2DvIMmHZEIyfbMEvvpUs2VzDi0HdTKvwJ4g2mjL96wOc/QalJOoJO5lehwrcdfeQcQntSbS3bd6kBj5ZMAA8iA/8bYjrEox8JVsxwN1TgGcrJrBuJ8K8Vh7DVs3mO8+3+CGC97t9W3C2RCp7iwh759vcEwcEtV9iKA3hISekXaN7aPefqcQRUvugO27XB6+yBeQtV5tPUksKgY9J9R+U7AHkqOmwCUxqZaE9rHerJ20G4HpV/mCHQ==";

    match dec_jwt(&secret.to_owned(), &content.access_token) {
        Some(c) => {
            HttpResponse::TemporaryRedirect()
            .header(header::LOCATION, format!("http://localhost:8080/index?username={}", c.preferred_username))
            .finish()
        }
        _ => HttpResponse::NonAuthoritativeInformation()
                .content_type("text/plain; charset=utf-8")
                .body("invalid token."),
    }
}


fn dec_jwt(secret: &String, jwt: &String) -> Option<Claims> {
    let TokenSlices {message, signature, header, claims } = raw::split_token(jwt).unwrap();
    log::info!("{:?}", raw::decode_json_token_slice(header).unwrap());

    let value = raw::decode_json_token_slice(claims).unwrap().to_string();
    let claims = serde_json::from_str::<Claims>(&value).unwrap();

    Some(Claims { 
        preferred_username: claims.preferred_username,
        iat: claims.iat,
        exp: claims.exp
    })

    // TODO:　検証する
    // match jsonwebtoken::decode::<Claims>(
    //     &jwt,
    //     &jsonwebtoken::DecodingKey::from_base64_secret(secret.as_ref()).unwrap(),
    //     &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
    // ) {
    //     Ok(c) => Some(c.claims),
    //     _ => Some(Claims { preferred_username: "aa".to_string(), iat: 23, exp: 23 })//Option::None,
    // }
}