//! A command-line tool to repair locked files using the `netfs-unlker` library.
//!
//! This tool uses `clap` for command-line argument parsing and `log` for logging.
//! It provides an option to specify a single file or a directory containing multiple files
//! for repair operations. The actual repair functions are hypothetically provided by the `netfs-unlker` library.

use clap::Parser;
use log::LevelFilter;
use log::{error, info};
use netfs_unlker;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::process;

/// Command-line interface definition.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to locked file.
    /// Specify this using `-f <FILE>` or `--file <FILE>`.
    /// If specified, the program will attempt to repair the locked file.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Path to a directory containing locked files.
    /// Specify this using `-d <DIRECTORY>` or `--directory <DIRECTORY>`.
    /// If specified, the program will attempt to repair all locked files within the directory.
    #[arg(short, long, value_name = "DIRECTORY")]
    directory: Option<PathBuf>,

    /// Recursively search for locked files within the specified directory.
    /// Specify this using `-r` or `--recursive`.
    #[arg(short, long, value_name = "RECURSIVE", default_value = "false")]
    recursive: bool,

    /// Enable verbose output.
    /// Specify this using `-v` or `--verbose`.
    #[arg(short, long, value_name = "VERBOSE", default_value = "false")]
    verbose: bool,
}

fn main() {
    // Parse command-line arguments.
    let args = Cli::parse();

    let default_log_level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    // Initialize the logger.
    SimpleLogger::new()
        .with_level(default_log_level)
        .init()
        .unwrap();

    // Handle the specified command-line options.
    match (&args.file, &args.directory, &args.recursive) {
        // Single file specified.
        (Some(file_path), None, _) => {
            info!("Processing single file: {}", file_path.display());
            // Attempt to repair the specified file.
            if let Err(e) = netfs_unlker::repair_file(file_path) {
                error!("Failed to repair file: {}", e);
                process::exit(1);
            }
        }
        // Directory specified.
        (None, Some(directory_path), &recursive) => {
            info!("Processing directory: {}", directory_path.display());
            // Attempt to repair all files within the specified directory.
            if let Err(e) = netfs_unlker::repair_files_in_directory(directory_path, recursive) {
                error!("Failed to repair files in directory: {}", e);
                process::exit(1);
            }
        }
        // Neither a single file nor a directory specified.
        _ => {
            error!("Please specify either a file path or a directory path");
            println!("Usage: -f <file path> or -d <directory path>");
            process::exit(1);
        }
    }
}
