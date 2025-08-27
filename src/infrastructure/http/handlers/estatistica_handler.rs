use actix_web::HttpResponse;

use crate::{application::estatistica_service, infrastructure::db::transacao_repo::Repo};

pub async fn estatisticas(repo: Repo) -> HttpResponse {
    HttpResponse::Ok().json(estatistica_service::estatisticas(&repo))
}
