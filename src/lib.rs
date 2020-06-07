mod usages;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
use usages::Usages;

/// All errors the library's public interface can return.
#[derive(Error, Debug)]
pub enum UsageTrackerError {
    /// The loading (most likely parsing) of a RON file failed. Contains the root cause.
    #[error("RON file could not be loaded")]
    FileLoadErrorRon(#[source] ron::Error),
    /// Tried to add a new object to keep track of, but object with same name is already tracked.
    #[error("object \"{name}\" is already tracked")]
    ObjectAlreadyTracked { name: String },
}

/// A struct that keeps the records for all tracked objects.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UsageInformation {
    usage_information: BTreeMap<String, Usages>,
}

impl UsageInformation {
    pub fn add(&mut self, name: String) -> Result<(), UsageTrackerError> {
        if self.usage_information.contains_key(&name) {
            return Err(UsageTrackerError::ObjectAlreadyTracked { name: name });
        }

        self.usage_information.insert(name, Usages::new());
        Ok(())
    }

    /// Loads a UsageInformation object from a RON file.
    ///
    /// With v0.2, the data layout was changed. To make the transition from v0.1 easier for users,
    /// this function was created. It is able to read the RON files produced by v0.1 and convert
    /// them into the data structure of v0.2.
    ///
    /// **IMPORTANT:** if it still exists by then, v1.0 will see this function removed.
    #[deprecated(
        since = "0.2",
        note = "please only use this function if you have to load files from v0.1"
    )]
    pub fn load_usage_information_from_ron_file<R>(rdr: R) -> Result<Self, UsageTrackerError>
    where
        R: std::io::Read,
    {
        Ok(Self {
            usage_information: ron::de::from_reader(rdr)
                .or_else(|e| return Err(UsageTrackerError::FileLoadErrorRon(e)))?,
        })
    }

    /// Creates a new, empty UsageInformation object.
    pub fn new() -> Self {
        Self {
            usage_information: BTreeMap::new(),
        }
    }
}
