pub use chrono::{DateTime, Duration, NaiveDateTime, Utc};
pub use serde::{Deserialize, Serialize};

pub const AUTH_HEADER: &str = "EBOBO-FINGERPRINT";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub fighter: Option<String>,
    pub rank: Option<i32>,
    pub greet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arena {
    pub total: i32,
    pub queue: i32,
    pub rank: i32,
    pub you: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fighter(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice(pub String);
