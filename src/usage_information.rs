use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Keeps track of the usages of something.
#[derive(Debug, Deserialize, Serialize)]
pub struct UsageInformation {
    /// All past usages of something in UTC time.
    usages: Vec<DateTime<Utc>>,
}

impl UsageInformation {
    pub fn new() -> Self {
        UsageInformation { usages: vec![] }
    }

    pub fn get_usages(&self) -> &Vec<DateTime<Utc>> {
        &self.usages
    }

    /// Adds a usage at the current time.
    pub fn use_now(&mut self) {
        self.usages.push(Utc::now());
    }
}
