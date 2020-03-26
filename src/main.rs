/// Information on the usage of things.
mod usage_information;

use std::{fs, path::Path};
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
    let path = "/tmp/test.json";
    let path_bak = format!("{}.bak", path);
    let path = Path::new(path);
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
