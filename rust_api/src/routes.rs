use actix_web::web;
use crate::controllers::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    // cfg.route("/index", web::post().to(index::index));
    // cfg.route("/index", web::get().to(index::get_index));


    cfg.service(web::resource("/index")
        .route(web::post().to(index::index))
        .route(web::get().to(index::get_index))
    );

    cfg.service(web::resource("/index/{user_id}")
        .route(web::get().to(index::id_get)));

    cfg.service(web::resource("/getcode")
        .route(web::get().to(getcode::get_code)));

    cfg.service(web::resource("/token")
        .route(web::post().to(token::token)));

    cfg.service(web::resource("/login")
        .route(web::get().to(login::login)));
}
