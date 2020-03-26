/// Information on the usage of things.
mod usage_information;

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

    let cfg = Opt::from_args();

    let mut things = Things::new();

    match cfg {
        Opt::Add { name } => {
            things.insert(name, UsageInformation::new());
        },
    }

    dbg!(things);
}
