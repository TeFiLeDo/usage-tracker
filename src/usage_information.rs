use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A map of all tracked objects combined with their usages.
pub type UsageInformation = BTreeMap<String, Usages>;

/// Keeps track of the usages of something.
#[derive(Debug, Deserialize, Serialize)]
pub struct Usages {
    /// All past usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}
