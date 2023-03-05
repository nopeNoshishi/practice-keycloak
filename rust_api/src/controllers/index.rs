use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
}



pub async fn index(data: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", data.username))
}

pub async fn get_index(data: web::Query<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", data.username))
}

pub async fn id_get(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get {}", user_id))
}
