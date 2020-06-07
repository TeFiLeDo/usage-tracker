mod usages;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
use usages::Usages;

/// All errors the library's public interface can return.
#[derive(Error, Debug)]
pub enum UsageTrackerError {
    /// The loading (most likely parsing) of a JSON file failed. Contains the root cause.
    #[error("JSON file could not be loaded")]
    FileLoadErrorJson(#[source] serde_json::Error),

    /// The loading (most likely parsing) of a RON file failed. Contains the root cause.
    #[error("RON file could not be loaded")]
    FileLoadErrorRon(#[source] ron::Error),
}

/// A struct that keeps the records for all tracked objects.
#[derive(Debug, Deserialize, Serialize)]
pub struct UsageInformation {
    usage_information: BTreeMap<String, Usages>,
}

impl UsageInformation {
    /// Loads a UsageInformation object from a JSON file.
    pub fn load_usage_information_from_json_reader<R>(rdr: R) -> Result<Self, UsageTrackerError>
    where
        R: std::io::Read,
    {
        serde_json::from_reader::<_, Self>(rdr)
            .or_else(|e| return Err(UsageTrackerError::FileLoadErrorJson(e)))
    }

    /// Loads a UsageInformation object from a RON file.
    ///
    /// With v0.2, the default data format was changed to JSON. In order to provide a simple API for
    /// users of the library, the UsageInformation type was changed from a BTreeMap to be its own
    /// struct. Since RON is only supported to make migration from v0.1 trivial for users, this
    /// means:
    /// 1. The JSON and RON loading methods are not the same, since the RON method must convert the
    ///    data to the new format.
    /// 2. v0.2 is no longer capable of writing RON files, since that would make the migration
    ///    process much harder to deal with.
    ///
    /// **IMPORTANT:** if it still exists by then, v1.0 will see all RON support removed.
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
