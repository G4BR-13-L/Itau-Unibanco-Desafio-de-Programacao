use actix_web::{HttpResponse, web};

use crate::{
    application::dto::transacao_dto::{CreateTransacaoRequest, TransacaoResponse},
    domain::transacao::Transacao,
};

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello transacoes")
}

pub async fn create(transacao: web::Json<CreateTransacaoRequest>) -> HttpResponse {
    let new_transacao = Transacao {
        valor: transacao.valor.clone(),
        data_hora: transacao.dataHora.clone(),
    };
    HttpResponse::Ok().json(TransacaoResponse::from(new_transacao))
}
