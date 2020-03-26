/// Information on the usage of things.
mod usage_information;

use standard_paths::{LocationType, StandardPaths};
use std::{
    fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use usage_information::UsageInformation;

/// A hashmap of named things whose usage can be tracked.
type Things = std::collections::HashMap<String, UsageInformation>;

#[derive(Debug, StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
/// The cli.
enum Opt {
    /// Add a new thing to keep track of.
    Add {
        ///The name of the new thing.
        name: String,
    },
}

fn main() {
    // setup human panic crate
    human_panic::setup_panic!(human_panic::Metadata {
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
    });

    // get command line options
    let cfg = Opt::from_args();

    // setup paths
    let sp = StandardPaths::new();
    let path_base = sp
        .writable_location(LocationType::AppDataLocation)
        .expect("No standard path found");
    if !path_base.exists() {
        fs::create_dir_all(&path_base).expect("Unable to create data directory");
    }
    let mut path = PathBuf::new();
    path.push(&path_base);
    path.push("default.json");
    let mut path_bak = PathBuf::new();
    path_bak.push(&path);
    path_bak.push(".bak");
    let path = Path::new(&path);
    let path_bak = Path::new(&path_bak);

    // setup things variable
    let mut things: Things = if path.exists() {
        serde_json::from_slice(fs::read(&path).expect("Unable to read data").as_slice())
            .expect("Unable to deserialize data")
    } else {
        Things::new()
    };

    // do work
    match cfg {
        Opt::Add { name } => {
            things.insert(name, UsageInformation::new());
        }
    }

    // save data
    {
        if path_bak.exists() {
            fs::remove_file(&path_bak).expect("Unable to delete old backup");
        }

        if path.exists() {
            fs::rename(&path, &path_bak).expect("Unable to move old data to backup location");
        }

        fs::write(
            &path,
            serde_json::to_vec(&things)
                .expect("Unable to serialize data")
                .as_slice(),
        )
        .expect("Unable to write data");
    }
}
