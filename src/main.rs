use anyhow::{anyhow, Context, Result};
use atty::Stream;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};
use human_panic::setup_panic;
use standard_paths::{LocationType, StandardPaths};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use usage_tracker::*;

const PATH_CONVERT_ERROR: &str =
    "could not convert file name for other error message. WTF have you done?!";
const JSON_FORMAT_ERROR: &str = "could not serialize JSON output";

/// The CLI.
#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Opt {
    /// The commands.
    #[structopt(subcommand)]
    cmd: Commands,
    /// The data file to use.
    ///
    /// If the file doesn't exist, it will be treated as an empty file and if an object is added, it
    /// will be saved at the location.
    ///
    /// Supported file formats:
    /// - json
    ///
    /// Warning: even if RON support is added at some point, you won't be able to read files from
    /// v0.1 with it, because those files have a different file format.
    #[structopt(parse(from_os_str), verbatim_doc_comment)]
    data_file: Option<PathBuf>,
    /// If a change is made, don't keep a backup of the original data file.
    #[structopt(long)]
    no_backup: bool,
}

/// All possible commands.
#[derive(Debug, StructOpt)]
enum Commands {
    /// Add a new object to keep track of.
    Add {
        /// The name of the new object.
        name: String,
    },

    /// Remove **all** objects permanently.
    Clear,

    /// List all currently tracked objects.
    List {
        /// Print all usage dates in addition to the objects names.
        #[structopt(long, short)]
        verbose: bool,
    },

    /// Remove usages from an object.
    Prune {
        /// Remove all usages before this point in time. If not specified, all usages are removed.
        ///
        /// Can be in one of these formats:
        ///
        /// - 'dd.MM.yyyy': if this format is used, the timezone is set as the local timezone.
        /// - 'yyyy-MM-ddThh:mm:ss': if this format is used, the timezone is set as the local
        ///                          timezone. Intended for use by other programs, but humans should
        ///                          be able to use it too.
        /// - 'yyyy-MM-ddThh:mm:ss+oh:om': this format allows you to specify the timezone yourself.
        ///                                `oh` is the offset hour value, 'om' the offset minute
        ///                                value. Intended for use by other programs.
        #[structopt(short, long, parse(try_from_str = parse_date), verbatim_doc_comment)]
        before: Option<DateTime<Utc>>,
        /// The name of the object to modify.
        name: String,
    },

    /// Remove a currently tracked object permanently.
    Remove {
        /// The name of the object to remove.
        name: String,
    },

    /// Show all usages of a single object.
    Show {
        /// The name of the object.
        name: String,
    },

    /// Record a new usage of an object.
    Use {
        /// Add the object if it isn't tracked yet.
        #[structopt(long)]
        add_if_new: bool,
        /// The name of the object that was used.
        name: String,
    },

    /// Show a prediction of the number of uses of an object within a time frame.
    Usage {
        /// The name of the object to predict for.
        name: String,

        /// The duration to consider.
        duration: i64,

        ///The type of duration to consider
        ///
        /// Allowed values:
        /// - y...year
        /// - M...month
        /// - w...week
        /// - d...day
        /// - h...hour
        /// - m...minute
        /// - s...second
        #[structopt(verbatim_doc_comment)]
        duration_type: char,
    },
}

fn main() -> Result<()> {
    // setup panic handler
    setup_panic!(Metadata {
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
    });

    // parge arguments
    let opt = Opt::from_args();

    // load data
    let initial_info = match &opt.data_file {
        Some(df) => load_from_file(&df)?,
        None => load_from_default_files()?,
    };
    let mut info = initial_info.clone();

    // handle commands
    match opt.cmd {
        Commands::Add { name } => info.add(&name)?,
        Commands::Clear => info.clear(),
        Commands::List { verbose } => {
            if info.list_verbose().len() == 0 {
                return Err(anyhow!("no objects are currently tracked"));
            }

            if !verbose {
                let data = info.list();

                if atty::is(Stream::Stdout) {
                    for (i, k) in data.iter().enumerate() {
                        println!("{}: {}", i, k);
                    }
                } else {
                    println!(
                        "{}",
                        serde_json::to_string(&data).context(JSON_FORMAT_ERROR)?
                    );
                }
            } else {
                let data = info.list_verbose();

                if atty::is(Stream::Stdout) {
                    for (i, (k, v)) in data.iter().enumerate() {
                        println!("{}: {}", i, k);
                        for u in v.list() {
                            println!("   {}", u.with_timezone(&chrono::Local));
                        }
                    }
                } else {
                    let mut output = Vec::new();
                    for (k, v) in data.iter() {
                        output.push(serde_json::json!({"name": k, "usages": v.list()}));
                    }
                    println!(
                        "{}",
                        serde_json::to_string(&output).context(JSON_FORMAT_ERROR)?
                    );
                }
            }
        }
        Commands::Prune { before, name } => info.prune(&name, &before)?,
        Commands::Remove { name } => info.remove(&name),
        Commands::Show { name } => {
            let data = (info.usages(&name)?).list();
            if atty::is(Stream::Stdout) {
                for u in data {
                    println!("{}", u.with_timezone(&chrono::Local));
                }
            } else {
                println!(
                    "{}",
                    serde_json::to_string(&data).context(JSON_FORMAT_ERROR)?
                );
            }
        }
        Commands::Use { add_if_new, name } => info.record_use(&name, add_if_new)?,
        Commands::Usage {
            name,
            duration,
            duration_type,
        } => {
            let d = match duration_type {
                'y' => Duration::days(duration * 365),
                'M' => Duration::days(duration * 30),
                'w' => Duration::weeks(duration),
                'd' => Duration::days(duration),
                'h' => Duration::hours(duration),
                'm' => Duration::minutes(duration),
                's' => Duration::seconds(duration),
                _ => {
                    return Err(anyhow!("duration type '{}' doesn't exist", duration_type));
                }
            };

            let data = info.usage(&name, &d)?;
            if atty::is(Stream::Stdout) {
                println!("{}", data);
            } else {
                println!("{}", serde_json::json!({"value": data}));
            }
        }
    }

    // if data changed, safe new data
    if info != initial_info {
        match &opt.data_file {
            Some(df) => save_to_file(&info, &df, !opt.no_backup)?,
            None => save_to_default_file(&info, !opt.no_backup)?,
        }
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

/// Loads usage information from a file.
///
/// The file format is decided on basis of the file extension. Currently supported formats:
/// - JSON: `.json`
fn load_from_file(path: &PathBuf) -> Result<UsageInformation> {
    let fmt = match path.extension() {
        Some(e) => match e.to_str().context("could not parse file name extension")? {
            "json" => "JSON",
            _ => {
                return Err(anyhow!(
                    "\"{}\" is not a supported file format",
                    e.to_str().context(PATH_CONVERT_ERROR)?
                ))
            }
        },
        None => return Err(anyhow!("file format not specified")),
    };

    if !path.exists() {
        return Ok(UsageInformation::new());
    }

    let file = File::open(Path::new(&path)).context(format!(
        "could not open file: {}",
        path.to_str().context(PATH_CONVERT_ERROR)?
    ))?;

    match fmt {
        "JSON" => serde_json::from_reader(file),
        _ => panic!("internal format value changed"),
    }
    .context(format!(
        "could not parse {} file: {}",
        fmt,
        path.to_str().context(PATH_CONVERT_ERROR)?
    ))
}

/// Parses a &str into a DateTime<Utc>.
///
/// Tries different formats described by the documentation for the `prune -v` command parameter.
fn parse_date(src: &str) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    if src.len() == "dd.MM.yyyy".len() {
        let d = NaiveDate::parse_from_str(src, "%d.%m.%Y")
            .context(format!("could not parse local date: {}", src))?;
        return Ok(Utc
            .from_local_datetime(
                &d.and_hms_opt(0, 0, 0)
                    .ok_or(anyhow!("could not convert to utc: {}", d))?,
            )
            .unwrap());
    } else if src.len() == "yyyy-MM-ddThh:mm:ss".len() {
        let dt: NaiveDateTime = src
            .parse()
            .context(format!("could not pares local datetime: {}", src))?;

        let dtu = chrono::Local.from_local_datetime(&dt).unwrap();

        return Ok(dtu.into());
    } else {
        return Ok(src
            .parse()
            .context(format!("could not parse datetime: {}", src))?);
    }
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

    save_to_file(ui, &path, backup)
}

/// Saves the provided UsageInformation to a default file. The default file is the first file listed
/// in the documentation of `load_from_default_files()`.
///
/// The parameter `backup` specifies whether or not the function will create a backup of the
/// original file (if one exists), before overwriting it. This backup is very simple, it's literally
/// adding `.bak` to the original files name. If a file with that name already exists, it is
/// deleted.
fn save_to_file(ui: &UsageInformation, path: &PathBuf, backup: bool) -> Result<()> {
    let fmt = match path.extension() {
        Some(e) => match e.to_str().context("could not parse file name extension")? {
            "json" => "JSON",
            _ => {
                return Err(anyhow!(
                    "\"{}\" is not a supported file format",
                    e.to_str().context(PATH_CONVERT_ERROR)?
                ))
            }
        },
        None => return Err(anyhow!("file format not specified")),
    };

    if backup {
        // get backup path
        let mut backup_path = PathBuf::new();
        backup_path.push(&path);
        let backup_ext = backup_path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + ".bak";
        backup_path.set_extension(backup_ext);

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

    match fmt {
        "JSON" => serde_json::to_writer_pretty(file, ui),
        _ => panic!("internal format value changed"),
    }
    .context(format!(
        "could not parse {} file: {}",
        fmt,
        path.to_str().context(PATH_CONVERT_ERROR)?
    ))
}
