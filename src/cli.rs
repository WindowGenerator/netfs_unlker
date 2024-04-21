//! A command-line tool to repair locked files using the `fcunlock` library.
//!
//! This tool uses `clap` for command-line argument parsing and `log` for logging.
//! It provides an option to specify a single file or a directory containing multiple files
//! for repair operations. The actual repair functions are hypothetically provided by the `fcunlock` library.

use clap::Parser;
use log::{error, info};
use std::path::PathBuf;
use std::process;
use simple_logger::SimpleLogger;

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
}

fn main() {
    // Initialize the logger.
    SimpleLogger::new().init().unwrap();
    
    // Parse command-line arguments.
    let args = Cli::parse();

    // Handle the specified command-line options.
    match (&args.file, &args.directory) {
        // Single file specified.
        (Some(file_path), None) => {
            info!("Processing single file: {}", file_path.display());
            // Attempt to repair the specified file.
            if let Err(e) = fcunlock::repair_file(file_path) {
                error!("Failed to repair file: {}", e);
                process::exit(1);
            }
        },
        // Directory specified.
        (None, Some(directory_path)) => {
            info!("Processing directory: {}", directory_path.display());
            // Attempt to repair all files within the specified directory.
            if let Err(e) = fcunlock::repair_files_in_directory(directory_path) {
                error!("Failed to repair files in directory: {}", e);
                process::exit(1);
            }
        },
        // Neither a single file nor a directory specified.
        _ => {
            error!("Please specify either a file path or a directory path");
            println!("Usage: -f <file path> or -d <directory path>");
            process::exit(1);
        }
    }
}