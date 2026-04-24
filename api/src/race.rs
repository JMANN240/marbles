use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Race {
    pub id: i64,
    pub level_id: i64,
    pub time: Option<DateTime<Utc>>,
}
