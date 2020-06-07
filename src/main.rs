use anyhow::{anyhow, Context, Result};
use standard_paths::{LocationType, StandardPaths};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use usage_tracker::*;

const PATH_CONVERT_ERROR: &str =
    "Failed to convert file name for other error message. WTF have you done?!";

fn main() -> Result<()> {
    let _ = load_default_files()?;

    Ok(())
}

/// Loads usage information from one of two default files.
///
/// The files are always tried in the same order, an later files are only tried when the former file
/// wasn't found, but not if any other error occurred. All files are within the OS-specific
/// application data directory:
/// 1. `usages.json`: this is also the file the program writes to by default.
/// 2. `default.ron`: this was the default file in 0.1, so 0.2 should be able to fall back to it.
fn load_default_files() -> Result<UsageInformation> {
    // get application data directory
    let sp = StandardPaths::new();
    let path_base = sp
        .writable_location(LocationType::AppDataLocation)
        .context("application data directory not found")?;

    let files = vec![("usages", true), ("default", false)];

    for (name, is_json) in files {
        let mut p = PathBuf::new();
        p.push(&path_base);
        p.push(name);
        p.set_extension(match is_json {
            true => "json",
            false => "ron",
        });

        if !p.exists() {
            continue;
        }

        if !p.is_file() {
            return Err(anyhow!(
                "found directory instead of file: {}",
                p.to_str().context(PATH_CONVERT_ERROR)?
            ));
        }

        let file = File::open(Path::new(&p)).context(format!(
            "could not open file: {}",
            p.to_str().context(PATH_CONVERT_ERROR)?
        ))?;

        return match is_json {
            true => load_usage_information_from_json_reader(file).context(format!(
                "could not load data from JSON file: {}",
                p.to_str().context(PATH_CONVERT_ERROR)?
            )),
            false => load_usage_information_from_ron_file(file).context(format!(
                "could not load data from RON file: {}",
                p.to_str().context(PATH_CONVERT_ERROR)?
            )),
        };
    }

    Ok(UsageInformation::new())
}
