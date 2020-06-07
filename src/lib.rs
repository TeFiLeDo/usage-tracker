mod usage_information;

use thiserror::Error;
pub use usage_information::UsageInformation;

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

/// Loads a UsageInformation object from a JSON file.
pub fn load_usage_information_from_json_reader<R>(
    rdr: R,
) -> Result<UsageInformation, UsageTrackerError>
where
    R: std::io::Read,
{
    serde_json::from_reader::<_, UsageInformation>(rdr)
        .or_else(|e| return Err(UsageTrackerError::FileLoadErrorJson(e)))
}

/// Loads a UsageInformation object from a RON file.
pub fn load_usage_information_from_ron_file<R>(
    rdr: R,
) -> Result<UsageInformation, UsageTrackerError>
where
    R: std::io::Read,
{
    ron::de::from_reader::<_, UsageInformation>(rdr)
        .or_else(|e| return Err(UsageTrackerError::FileLoadErrorRon(e)))
}
