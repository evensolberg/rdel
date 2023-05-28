use clap::parser::ValueSource;
use std::fs;
use std::{error::Error, path::Path};

mod cli;
mod utils;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build();

    // Set up logging
    let _logbuilder = utils::log_build(&cli_args);

    let files_to_delete = cli_args
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(std::string::String::as_str);
    log::trace!("files_to_delete: {files_to_delete:?}");

    let move_files = cli_args.value_source("move") == Some(ValueSource::CommandLine);
    let stop_on_error = cli_args.value_source("stop") == Some(ValueSource::CommandLine);
    let show_detail_info = cli_args.value_source("detail-off") != Some(ValueSource::CommandLine);
    let dry_run = cli_args.value_source("dry-run") == Some(ValueSource::CommandLine);
    let print_summary = cli_args.value_source("print-summary") == Some(ValueSource::CommandLine);
    log::debug!("move_files: {move_files}, stop_on_error: {stop_on_error}, show_detail_info: {show_detail_info}, dry_run: {dry_run}, print-summary: {print_summary}");

    if dry_run {
        log::info!("Dry-run starting.");
    }

    let mut total_file_count: usize = 0;
    let mut processed_file_count: usize = 0;
    let mut skipped_file_count: usize = 0;
    let mut total_file_size: u64 = 0;

    // Delete files

    for filename in files_to_delete {
        total_file_count += 1;

        let current_file_size = fs::metadata(Path::new(&filename))?.len();

        total_file_size += current_file_size;

        if show_detail_info {
            log::info!("Deleting: {filename} for {current_file_size} bytes.");
        }

        if dry_run {
            processed_file_count += 1;
        } else {
            match std::fs::remove_file(filename) {
                Ok(_) => {
                    processed_file_count += 1;
                }
                Err(err) => {
                    if stop_on_error {
                        return Err(format!(
                            "Error: {err}. Unable to remove file {filename}. Halting.",
                        )
                        .into());
                    }
                    log::warn!("Unable to remove file {filename}. Continuing.");
                    skipped_file_count += 1;
                } // Err
            } // match
        }
    } // for filename

    // Print summary information
    if print_summary {
        log::info!("Total files examined:        {total_file_count:5}");
        log::info!("Files removed:               {processed_file_count:5}");
        log::info!("Files skipped due to errors: {skipped_file_count:5}");
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
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}

/// Pretty-prints integer values;
/// Examples:
///
/// ```
/// assert_eq!(thousand_separated(10000), "10,000".to_string());
/// assert_eq!(thousand_separated(10000000), "10,000,000".to_string());
/// ```
pub fn thousand_separated<T>(val: T) -> String
where
    T: std::fmt::Display,
{
    let s = val.to_string();
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or("0"))
        .collect();
    let result: Vec<_> = chunks.join(",").bytes().rev().collect();
    let default = String::from("NaN");
    String::from_utf8(result).unwrap_or(default)
}
