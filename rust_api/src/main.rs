use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use rust_api::routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body(
                    "/"
                ) }),
            )
            // 別モジュールに配置する
            .configure(routes::routes)
            // Logのための設定
            .wrap(Logger::default())
    })
    .workers(6)
    .bind("localhost:8080")?
    .run()
    .await
}
