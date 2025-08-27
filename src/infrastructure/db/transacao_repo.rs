use actix_web::{HttpResponse, web};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

use crate::domain::transacao::Transacao;

pub type Repo = web::Data<Arc<RwLock<HashMap<Uuid, Transacao>>>>;

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
