use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct CreateTokenRequest {
    username: String,
    password: String
}

pub async fn create_user(data: web::Json<CreateTokenRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("username:{}\n password{}", &data.username, &data.password))


}