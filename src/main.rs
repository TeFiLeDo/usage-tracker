use anyhow::{anyhow, Context, Result};
use standard_paths::{LocationType, StandardPaths};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};
use usage_tracker::*;

const PATH_CONVERT_ERROR: &str =
    "Failed to convert file name for other error message. WTF have you done?!";

fn main() -> Result<()> {
    let init_info = load_from_default_files()?;
    let info = init_info.clone();

    if info != init_info {
        save_to_default_file(&info, true)?;
    }

    Ok(())
}

/// Loads usage information from one of two default files.
///
/// The files are always tried in the same order, an later files are only tried when the former file
/// wasn't found, but not if any other error occurred. All files are within the OS-specific
/// application data directory:
/// 1. `usages.json`: this is also the file the program writes to by default.
/// 2. `default.ron`: this was the default file in 0.1, so 0.2 should be able to fall back to it.
fn load_from_default_files() -> Result<UsageInformation> {
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
            true => serde_json::from_reader(file).context(format!(
                "could not parse JSON file: {}",
                p.to_str().context(PATH_CONVERT_ERROR)?
            )),
            false => UsageInformation::load_usage_information_from_ron_file(file).context(format!(
                "could not load data from RON file: {}",
                p.to_str().context(PATH_CONVERT_ERROR)?
            )),
        };
    }

    Ok(UsageInformation::new())
}

/// Saves the provided UsageInformation to a default file. The default file is the first file listed
/// in the documentation of `load_from_default_files()`.
///
/// The parameter `backup` specifies whether or not the function will create a backup of the
/// original file (if one exists), before overwriting it. This backup is very simple, it's literally
/// adding `.bak` to the original files name. If a file with that name already exists, it is
/// deleted.
fn save_to_default_file(ui: &UsageInformation, backup: bool) -> Result<()> {
    // get file path
    let sp = StandardPaths::new();
    let mut path = sp
        .writable_location(LocationType::AppDataLocation)
        .context("application data directory not found")?;
    path.push("usages");
    path.set_extension("json");

    if backup {
        // get backup path
        let mut backup_path = PathBuf::new();
        backup_path.push(&path);
        backup_path.set_extension("json.bak");

        // make sure backup path is clear
        if backup_path.exists() {
            fs::remove_file(&backup_path).context("couldn't clear backup file path")?;
        }

        // move old file
        if path.exists() {
            fs::rename(&path, &backup_path)
                .context("couldn't move old data file to backup location")?;
        }
    }

    // make sure path is clear
    if path.exists() {
        fs::remove_file(&path).context("couldn't clear data file path")?;
    }

    let file = File::create(Path::new(&path)).context(format!(
        "could not create file: {}",
        path.to_str().context(PATH_CONVERT_ERROR)?
    ))?;

    serde_json::to_writer_pretty(file, ui).context("couldn't format data to json")
}
