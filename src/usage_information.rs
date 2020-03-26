use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of something.
#[derive(Debug, Deserialize, Serialize)]
pub struct UsageInformation {
    /// All past usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}

impl UsageInformation {
    /// Remove all existing usages.
    pub fn clear(&mut self) {
        self.usages.clear();
    }

    /// Remove all usages before pit.
    pub fn clear_before(&mut self, pit: &DateTime<Utc>) {
        self.usages.retain(|u| u >= pit)
    }

    /// Get a reference to all the usages.
    pub fn get_usages(&self) -> &Vec<DateTime<Utc>> {
        &self.usages
    }

    /// Check if the thing wasn't used
    pub fn is_empty(&self) -> bool {
        self.usages.is_empty()
    }

    pub fn new() -> Self {
        UsageInformation { usages: vec![] }
    }

    /// Adds a usage at the current time.
    pub fn use_now(&mut self) {
        self.usages.push(Utc::now());
    }
}
