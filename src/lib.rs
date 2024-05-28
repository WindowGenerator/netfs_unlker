//! # File Repair Module
//!
//! This module contains functions for repairing files that are locked by the NetApp storage system.
//! It provides functionalities to handle single files or all files within a directory, managing file operations like copying, renaming, and unlocking.

extern crate libc;
extern crate log;

mod fcntl;

use log::{debug, error, info, warn};
use std::collections::VecDeque;
use std::fs::{copy, read_dir, rename, File};
use std::io::{self, Error};
use std::path::{Path, PathBuf};
use tempfile::tempdir;

const INVALID_UTF8: &'static str = "[Invalid UTF-8]";
const DEVIDER: &'static str = "#############################\n";

/// Repairs all files in the specified directory.
///
/// This function iterates over each file in the directory and attempts to repair it if it is locked.
/// It logs the process and returns an `io::Result<()>` indicating the outcome.
///
/// # Errors
///
/// Returns an `Err` if the specified directory path does not exist or if any file in the directory cannot be processed.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let dir_path = Path::new("/path/to/directory");
/// let recursive = false;
/// repair_files_in_directory(dir_path, recursive);
/// ```
pub fn repair_files_in_directory(directory_path: &Path, recursive: bool) -> io::Result<()> {
    if !directory_path.is_dir() {
        error!(
            "Such directory not found: ({})",
            directory_path.to_str().unwrap_or(INVALID_UTF8)
        );
        return Err(Error::from_raw_os_error(libc::ENOENT));
    }

    let mut buf: VecDeque<PathBuf> = VecDeque::new();
    buf.push_back(directory_path.to_path_buf());

    while let Some(queue_path) = buf.pop_front() {
        let paths: Vec<PathBuf> = read_dir(queue_path)?.map(|x| x.unwrap().path()).collect();
        for path in paths {
            if path.is_dir() && recursive {
                buf.push_back(path.clone());
            } else {
                unlock_netapp_file(&path)?;
            }
        }
    }

    Ok(())
}

/// Repairs a single file that is specified by the path.
///
/// If the file is locked, this function will attempt to unlock and restore it.
/// Logs are provided at each step to monitor the process.
///
/// # Errors
///
/// Returns an `Err` if the file does not exist or if the repair process fails.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let file_path = Path::new("/path/to/file.txt");
/// repair_file(file_path);
/// ```
pub fn repair_file(file_path: &Path) -> io::Result<()> {
    debug!("{}", DEVIDER);
    unlock_netapp_file(file_path)
}

/// Handles the actual unlocking and repairing process for a locked file.
///
/// Detailed logs are written to track the progress and any errors encountered during the process.
/// It involves copying the file to a temporary location, unlocking it, and then replacing the original file.
///
/// # Errors
///
/// Returns an `Err` if any step in the repair process fails, including invalid file path or access errors.
fn unlock_netapp_file(file_path: &Path) -> io::Result<()> {
    debug!(
        "Start unlocking file: ({})",
        file_path.to_str().unwrap_or(INVALID_UTF8)
    );

    let dir = tempdir()?;

    if !file_path.is_file() {
        warn!(
            "This is not a file name: ({})",
            file_path.to_str().unwrap_or(INVALID_UTF8)
        );
        return Ok(());
    }

    let netapp_file = File::open(file_path)?;

    if !fcntl::is_file_locked(&netapp_file) {
        info!(
            "File is not locked: ({})",
            file_path.to_str().unwrap_or(INVALID_UTF8)
        );
        return Ok(());
    }

    let tmp_file_name = file_path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|s| format!(".tmp.{}.neo4j", s))
        .ok_or_else(|| {
            error!(
                "Wrong format of file name ({})",
                file_path.to_str().unwrap_or(INVALID_UTF8)
            );
            Error::from_raw_os_error(libc::ENOENT)
        })?;

    let local_tmp_file_path = dir.path().join(&tmp_file_name);

    debug!(
        "Copy from netapp: netapp ({}) -> local ({})",
        file_path.to_str().unwrap_or(INVALID_UTF8),
        local_tmp_file_path.to_str().unwrap_or(INVALID_UTF8)
    );

    copy(&file_path, &local_tmp_file_path)?;

    let tmp_file = File::open(&local_tmp_file_path)?;
    debug!(
        "Unlock file: ({})",
        local_tmp_file_path.to_str().unwrap_or(INVALID_UTF8)
    );
    fcntl::unlock(&tmp_file)?;

    let netapp_tmp_file_path = file_path
        .parent()
        .ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?
        .join(&tmp_file_name);

    debug!(
        "Copy to back tmp path: local ({}) -> netapp ({})",
        local_tmp_file_path.to_str().unwrap_or(INVALID_UTF8),
        netapp_tmp_file_path.to_str().unwrap_or(INVALID_UTF8)
    );
    copy(&local_tmp_file_path, &netapp_tmp_file_path)?;
    debug!(
        "Atomic file rename: netapp({}) -> netapp ({})",
        netapp_tmp_file_path.to_str().unwrap_or(INVALID_UTF8),
        file_path.to_str().unwrap_or(INVALID_UTF8)
    );
    rename(&netapp_tmp_file_path, &file_path)?;

    info!(
        "Successfully unlocked: ({})",
        netapp_tmp_file_path.to_str().unwrap_or(INVALID_UTF8)
    );

    Ok(())
}
