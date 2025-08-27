use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    application::dto::transacao_dto::CreateTransacaoRequest,
    domain::transacao::Transacao,
    infrastructure::{
        db::transacao_repo::{self, Repo},
        error::ApiError,
    },
};

pub async fn delete_all(repo: &Repo) -> Result<(), ApiError> {
    transacao_repo::delete_all(repo);
    Ok(())
}

pub async fn create(
    repo: &Repo,
    transacao_request: CreateTransacaoRequest,
) -> Result<Transacao, ApiError> {
    let mut errors: Vec<String> = Vec::with_capacity(4);

    if transacao_request.dataHora.is_none() {
        errors.push("dataHora deve ser informada".into());
    }

    if transacao_request.valor.is_none() {
        errors.push("valor deve ser informado".into());
    }

    if errors.len() != 0 {
        return Err(ApiError::UnprocessableEntity {
            errors: errors.into(),
        });
    }

    let data_hora: DateTime<FixedOffset> =
        match DateTime::parse_from_rfc3339(transacao_request.dataHora.unwrap().as_str()) {
            Ok(dt) => dt,
            Err(_) => {
                errors.push("dataHora invalida".into());
                return Err(ApiError::UnprocessableEntity {
                    errors: errors.into(),
                });
            }
        };

    let transacao = Transacao {
        valor: transacao_request.valor.unwrap(),
        data_hora: data_hora,
    };

    if is_not_past(&transacao.data_hora) {
        errors.push("dataHora não pode ser do futuro".into());
    }

    if is_negative(&transacao.valor) {
        errors.push("Valor não deve ser negativo".into());
    }

    if errors.len() != 0 {
        return Err(ApiError::UnprocessableEntity {
            errors: errors.into(),
        });
    }
    return Ok(transacao_repo::save(&repo, transacao));
}

fn is_not_past(data_hora: &DateTime<FixedOffset>) -> bool {
    let data_hora_utc = data_hora.with_timezone(&Utc);
    data_hora_utc > Utc::now()
}

fn is_negative(valor: &f64) -> bool {
    *valor < 0.0
}
