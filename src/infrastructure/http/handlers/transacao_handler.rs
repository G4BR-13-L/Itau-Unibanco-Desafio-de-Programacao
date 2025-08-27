use actix_web::{Error, HttpResponse, web};

use crate::{
    application::{
        dto::transacao_dto::{CreateTransacaoRequest, TransacaoResponse},
        transacao_service,
    },
    domain::transacao::Transacao,
    infrastructure::{
        db::transacao_repo::{self, Repo},
        error::ApiError,
    },
};

pub async fn find_all(repo: Repo) -> HttpResponse {
    HttpResponse::Ok().json(transacao_repo::find_all(&repo))
}

pub async fn delete_all(repo: Repo) -> HttpResponse {
    match transacao_service::delete_all(&repo).await {
        Ok(()) => {
            println!("Todas as transações foram apagadas com sucessso.");
            return HttpResponse::Ok().finish();
        }
        Err(_) => {
            println!("Falha ao apagar ass transações.");
            return HttpResponse::BadRequest().finish();
        }
    }
}

pub async fn create(
    transacao_request: Result<web::Json<CreateTransacaoRequest>, Error>,
    repo: Repo,
) -> HttpResponse {
    match transacao_request {
        Ok(transacao_request) => {
            let transacao_request = transacao_request.into_inner();
            match transacao_service::create(&repo, transacao_request).await {
                Ok(transacao) => HttpResponse::Created().json(TransacaoResponse::from(transacao)),
                Err(e) => HttpResponse::UnprocessableEntity().json(e), // Aqui pode ser customizado também
            }
        }
        Err(_err) => {
            HttpResponse::BadRequest().finish() // 422
        }
    }
}
