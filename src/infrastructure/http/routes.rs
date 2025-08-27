use actix_web::web;

use crate::infrastructure::http::handlers::{estatistica_handler, transacao_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/transacoes").route("", web::get().to(transacao_handler::find_all)))
        .service(
            web::scope("/transacao")
                .route("", web::post().to(transacao_handler::create))
                .route("", web::delete().to(transacao_handler::delete_all)),
        )
        .service(web::scope("/estatistica").route("", web::get().to(estatistica_handler::hello)));
}
