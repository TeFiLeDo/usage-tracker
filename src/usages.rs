use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of an object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Usages {
    /// All recorded usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}
