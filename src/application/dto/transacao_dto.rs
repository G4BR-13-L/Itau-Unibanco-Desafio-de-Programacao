use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::domain::transacao::Transacao;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CreateTransacaoRequest {
    pub valor: Option<f64>,
    pub dataHora: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct TransacaoResponse {
    pub valor: f64,
    pub dataHora: DateTime<FixedOffset>,
}

impl From<Transacao> for TransacaoResponse {
    fn from(t: Transacao) -> Self {
        Self {
            valor: t.valor,
            dataHora: t.data_hora,
        }
    }
}
