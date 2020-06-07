use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of an object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Usages {
    /// All recorded usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}

impl Usages {
    pub fn clear(&mut self) {
        self.usages.clear();
    }

    /// Provides read access to all stored data.
    pub fn list(&self) -> &Vec<DateTime<Utc>> {
        &self.usages
    }

    /// Creates a new, empty Usages object.
    pub fn new() -> Self {
        Self { usages: Vec::new() }
    }

    pub fn prune(&mut self, before: DateTime<Utc>) {
        self.usages.retain(|u| u >= &before);
    }

    /// Records a new usage of an object.
    pub fn record_usage(&mut self) {
        self.usages.push(Utc::now());
    }
}
