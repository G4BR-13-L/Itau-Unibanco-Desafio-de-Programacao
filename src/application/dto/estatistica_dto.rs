use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct EstatisticaResponse {
    pub count: u32,
    pub sum: f64,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}
