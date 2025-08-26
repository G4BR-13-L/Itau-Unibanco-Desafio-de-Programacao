use actix_web::web;

use crate::infrastructure::http::handlers::{estatistica_handler, transacao_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/transacoes").route("", web::get().to(transacao_handler::hello)))
        .service(web::scope("/transacao").route("", web::post().to(transacao_handler::create)))
        .service(web::scope("/transacao").route("", web::delete().to(transacao_handler::hello)))
        .service(web::scope("/estatistica").route("", web::get().to(estatistica_handler::hello)));
}
