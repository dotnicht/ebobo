pub use serde::{Serialize, Deserialize};
pub use chrono::{DateTime, Utc};
pub use serde_json::*;

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub fingerprint: String,
    pub addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub fingerprint: String,
    pub is_active: bool,
    pub is_cat: bool,
    pub registered_at: DateTime<Utc>,
    pub locations: Vec<Location>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub hits: u32,
}