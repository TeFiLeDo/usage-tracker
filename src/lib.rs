mod usage_information;

use std::fs;
use std::path::Path;
use thiserror::Error;
pub use usage_information::UsageInformation;

#[derive(Error, Debug)]
pub enum UsageTrackerError {
    #[error("Data file not found: \"{file}\"")]
    FileNotFound { file: String },

    #[error("Failed to parse \"{file}\" as a RON-file")]
    FileParseErrorRon {
        file: String,
        #[source]
        source: ron::Error,
    },

    #[error(transparent)]
    FileReadError(#[from] std::io::Error),

    #[error("The standard path for application data could not be found.")]
    PathIsNotDefined {
        #[source]
        source: std::io::Error,
    },

    #[error("The provided path isn't a file: \"{path}\"")]
    PathIsNotFile { path: String },
}

pub fn load_usage_information_from_default() -> Result<UsageInformation, UsageTrackerError> {
    let sp = standard_paths::StandardPaths::new_with_names("usage-tracker", "TeFiLeDo");
    let mut path = sp
        .writable_location(standard_paths::LocationType::AppDataLocation)
        .or_else(|e| Err(UsageTrackerError::PathIsNotDefined { source: e }))?;

    path.push("default");
    path.set_extension("ron");

    load_usage_information_from_ron_file(Path::new(&path))
}

pub fn load_usage_information_from_ron_file(
    path: &Path,
) -> Result<UsageInformation, UsageTrackerError> {
    if !path.exists() {
        return Err(UsageTrackerError::FileNotFound {
            file: path.to_str().unwrap().to_owned(),
        });
    } else if !path.is_file() {
        return Err(UsageTrackerError::PathIsNotFile {
            path: path.to_str().unwrap().to_owned(),
        });
    }

    let file = fs::read(path).or_else(|e| return Err(UsageTrackerError::FileReadError(e)))?;
    let info: UsageInformation = ron::de::from_bytes(file.as_slice()).or_else(|e| {
        return Err(UsageTrackerError::FileParseErrorRon {
            file: path.to_str().unwrap().to_owned(),
            source: e,
        });
    })?;

    Ok(info)
}
