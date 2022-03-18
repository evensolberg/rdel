use clap::{App, Arg}; // Command line
use std::fs;
use std::{error::Error, path::Path};

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .long_about("Recursively delete files.")
        .arg(
            Arg::new("files")
                .value_name("FILE(S)")
                .help("One or more file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.pdf 2020*.pdf) are supported. Use the ** glob to recurse (eg. **/*.log). Note: Case sensitive.")
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .multiple_occurrences(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .takes_value(false)
                .hide(false),
        )
        .arg( // Dry-run
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .multiple_occurrences(false)
                .help("Iterate through the files and produce output without actually deleting anything.")
                .takes_value(false)
        )
        .arg( // Stop on error
            Arg::new("stop")
                .short('s')
                .long("stop-on-error")
                .multiple_occurrences(false)
                .help("If set, the program will stop if it encounters an error. If not, the program will attempt to continue if errors occur.")
                .takes_value(false)
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .multiple_occurrences(false)
                .help("Don't produce any output except errors while working.")
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .multiple_occurrences(false)
                .help("Print summary detail.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .multiple_occurrences(false)
                .help("Don't export detailed information about each file processed.")
                .takes_value(false)
        )
        .get_matches();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    // create a list of the files to delete
    for files_to_delete in cli_args.values_of("files").unwrap() {
        log::trace!("files_to_delete: {:?}", &files_to_delete);
    }

    let dry_run = cli_args.is_present("dry-run");
    if dry_run {
        log::info!("Dry-run starting.");
    }
    let stop_on_error = cli_args.is_present("stop");
    if stop_on_error {
        log::debug!("Stop on error flag set. Will stop if errors occur.");
    } else {
        log::debug!("Stop on error flag not set. Will attempt to continue in case of errors.");
    }

    let show_detail_info = !cli_args.is_present("detail-off");

    let mut total_file_count: usize = 0;
    let mut processed_file_count: usize = 0;
    let mut skipped_file_count: usize = 0;
    let mut total_file_size: u64 = 0;

    // Delete files

    for filename in cli_args.values_of("files").unwrap() {
        total_file_count += 1;

        let current_file_size = fs::metadata(Path::new(&filename))?.len();

        total_file_size += current_file_size;

        if dry_run || show_detail_info {
            log::info!("Deleting: {} for {} bytes.", &filename, current_file_size);
            processed_file_count += 1;
        }

        if !dry_run {
            match std::fs::remove_file(&filename) {
                Ok(_) => {
                    processed_file_count += 1;
                }
                Err(err) => {
                    if stop_on_error {
                        return Err(format!(
                            "Error: {}. Unable to remove file {}. Halting.",
                            err, &filename,
                        )
                        .into());
                    } else {
                        log::warn!("Unable to remove file {}. Continuing.", &filename,);
                        skipped_file_count += 1;
                    } // if stop_on_error
                } // Err
            } // match
        } // if !dry_run
    } // for filename

    // Print summary information
    if cli_args.is_present("print-summary") {
        log::info!("Total files examined:        {:5}", total_file_count);
        log::info!("Files removed:               {:5}", processed_file_count);
        log::info!("Files skipped due to errors: {:5}", skipped_file_count);
        log::info!(
            "Bytes freed:                 {:>}",
            thousand_separated(total_file_size)
        );
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace("\"", ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}

/// Pretty-prints `usize` values;
/// Examples:
///
/// ```
/// assert_eq!(thousand_separated(10000), "10,000".to_string());
/// assert_eq!(thousand_separated(10000000), "10,000,000".to_string());
/// ```
pub fn thousand_separated(val: u64) -> String {
    let s = val.to_string();
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    let result: Vec<_> = chunks.join(",").bytes().rev().collect();
    String::from_utf8(result).unwrap()
}
