use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of an object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Usages {
    /// All recorded usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}

impl Usages {
    /// Provides read access to all stored data.
    pub fn list(&self) -> &Vec<DateTime<Utc>> {
        &self.usages
    }

    /// Creates a new, empty Usages object.
    pub fn new() -> Self {
        Self { usages: Vec::new() }
    }

    /// Records a new usage of an object.
    pub fn record_usage(&mut self) {
        self.usages.push(Utc::now());
    }
}
