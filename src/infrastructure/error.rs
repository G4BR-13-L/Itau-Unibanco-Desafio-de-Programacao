use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    #[error("Bad request: {errors:?}")]
    UnprocessableEntity { errors: Vec<String> },
}
