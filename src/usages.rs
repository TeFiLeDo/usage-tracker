use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of an object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Usages {
    /// All recorded usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}

impl Usages {
    /// Creates a new, empty Usages object.
    pub fn new() -> Self {
        Self { usages: Vec::new() }
    }

    pub fn record_usage(&mut self) {
        self.usages.push(Utc::now());
    }
}
