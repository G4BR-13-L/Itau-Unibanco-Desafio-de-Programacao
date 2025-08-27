use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transacao {
    pub valor: f64,
    pub data_hora: DateTime<FixedOffset>,
}
