/// Information on the usage of things.
mod usage_information;

use standard_paths::{LocationType, StandardPaths};
use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};
use structopt::StructOpt;
use usage_information::UsageInformation;

/// A hashmap of named things whose usage can be tracked.
type Things = std::collections::BTreeMap<String, UsageInformation>;

#[derive(Debug, StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
/// The cli.
struct Opt {
    #[structopt(subcommand)]
    /// The subcommands.
    cmd: Commands,
    #[structopt(long)]
    /// Use UTC instead of the local timezone.
    utc: bool,
}

#[derive(Debug, StructOpt)]
/// All possible subcommands.
enum Commands {
    /// Add a new thing to keep track of.
    Add {
        ///The name of the new thing.
        name: String,
    },
    /// List all existing things.
    List,
    /// Remove a thing from the tracker.
    Remove {
        /// The name of the thing to remove.
        name: String,
    },
    /// Add a new usage record to a thing.
    Use {
        /// The name of the thing to use.
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
    path.push("default");
    path.set_extension("ron");
    let mut path_bak = PathBuf::new();
    path_bak.push(&path);
    path_bak.set_extension("ron.bak");
    let path = Path::new(&path);
    let path_bak = Path::new(&path_bak);

    // setup things variable
    let mut things: Things = if path.exists() {
        ron::de::from_bytes(fs::read(&path).expect("Unable to read data").as_slice())
            .expect("Unable to deserialize data")
    } else {
        Things::new()
    };

    let mut change = false;

    // do work
    match cfg.cmd {
        Commands::Add { name } => {
            change = true;
            things.insert(name, UsageInformation::new());
        }
        Commands::List => {
            for (pos, (name, usage)) in things.iter().enumerate() {
                println!("{}: {}", pos, name);
                for u in usage.get_usages() {
                    if (cfg.utc) {
                        println!("   used at: {}", u);
                    } else {
                        println!("   used at: {}", u.with_timezone(&chrono::Local));
                    }
                }
            }
        }
        Commands::Remove { name } => {
            if things.contains_key(&name) {
                change = true;
                things.remove(&name);
            } else {
                println!("No thing named \"{}\" exists. Ignoring command.", name);
            }
        }
        Commands::Use { name } => {
            if things.contains_key(&name) {
                change = true;
                things.entry(name).and_modify(|e| {
                    e.use_now();
                });
            } else {
                eprintln!("No thing named \"{}\" exists. Ignoring command.", name);
                exit(exitcode::DATAERR);
            }
        }
    }

    // save data
    if change {
        if path_bak.exists() {
            fs::remove_file(&path_bak).expect("Unable to delete old backup");
        }

        if path.exists() {
            fs::rename(&path, &path_bak).expect("Unable to move old data to backup location");
        }

        fs::write(
            &path,
            ron::ser::to_string(&things)
                .expect("Unable to serialize data")
                .as_bytes(),
        )
        .expect("Unable to write data");
    }
}
