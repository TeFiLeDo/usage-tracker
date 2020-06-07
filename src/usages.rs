use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of an object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Usages {
    /// All recorded usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}
