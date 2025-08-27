use std::f64;

use crate::{
    application::dto::estatistica_dto::EstatisticaResponse,
    domain::transacao::Transacao,
    infrastructure::db::transacao_repo::{self, Repo},
};

pub fn estatisticas(repo: &Repo) -> EstatisticaResponse {
    let transacoes = transacao_repo::find_last_minute_transacoes(&repo);

    if transacoes.is_empty() {
        return EstatisticaResponse {
            count: 0,
            sum: 0.0,
            avg: 0.0,
            min: 0.0,
            max: 0.0,
        };
    }

    let mut count = 0u32;
    let mut sum = 0.0f64;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for t in &transacoes {
        count += 1;
        sum += t.valor;
        if t.valor < min {
            min = t.valor;
        }
        if t.valor > max {
            max = t.valor;
        }
    }

    EstatisticaResponse {
        count,
        sum,
        avg: sum / count as f64,
        min,
        max,
    }
}
