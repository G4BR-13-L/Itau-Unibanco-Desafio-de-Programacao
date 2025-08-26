use actix_web::web;

use crate::infrastructure::http::handlers::transacao_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/transacoes").route("", web::get().to(transacao_handler::hello)));
}
