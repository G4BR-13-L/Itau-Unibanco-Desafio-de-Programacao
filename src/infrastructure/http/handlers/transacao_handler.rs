use actix_web::{HttpResponse, web};

use crate::{
    application::dto::transacao_dto::{CreateTransacaoRequest, TransacaoResponse},
    domain::transacao::Transacao,
    infrastructure::db::transacao_repo::{self, Repo},
};

pub async fn find_all(repo: Repo) -> HttpResponse {
    HttpResponse::Ok().json(transacao_repo::find_all(&repo))
}

pub async fn create(transacao: web::Json<CreateTransacaoRequest>, repo: Repo) -> HttpResponse {
    let new_transacao = Transacao {
        valor: transacao.valor.clone(),
        data_hora: transacao.dataHora.clone(),
    };

    let saved_transacao = transacao_repo::save(&repo, new_transacao);

    HttpResponse::Ok().json(TransacaoResponse::from(saved_transacao))
}
