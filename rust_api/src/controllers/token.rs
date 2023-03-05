use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    refresh_token: String,
    id_token: String,
}

pub async fn token(data: web::Json<Token>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Access token {}\nRefresh token {}\n Id token {}"
        ,data.access_token, data.refresh_token, data.id_token)
    )
}
