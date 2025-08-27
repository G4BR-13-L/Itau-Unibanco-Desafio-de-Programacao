use actix_web::{HttpResponse, web};
use chrono::Utc;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use uuid::Uuid;

use crate::domain::transacao::Transacao;

pub type Repo = web::Data<Arc<RwLock<HashMap<Uuid, Transacao>>>>;

pub fn delete_all(repo: &Repo) {
    let mut map = repo.write().unwrap();
    map.clear();
}

pub fn save(repo: &Repo, transacao: Transacao) -> Transacao {
    let mut map = repo.write().unwrap();
    let id = Uuid::new_v4();
    map.insert(id, transacao.clone());
    transacao
}

pub fn find_all(repo: &Repo) -> Vec<Transacao> {
    let map = repo.read().unwrap();
    map.values().cloned().collect()
}

pub fn find_last_minute_transacoes(repo: &Repo) -> Vec<Transacao> {
    use chrono::Duration;
    let one_minute_ago = Utc::now() - Duration::minutes(1); // CERTO

    // Pega o lock da HashMap
    let transacoes = repo.read().unwrap();

    transacoes
        .values()
        .cloned()
        .filter(|t| t.data_hora > one_minute_ago.with_timezone(t.data_hora.offset()))
        .collect()
}
