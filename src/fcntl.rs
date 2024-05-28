//! A module for handling file locks in a Unix-like environment.
//!
//! This module provides functions to lock and unlock files, and to handle lock errors.

extern crate libc;

use std::fs::File;
use std::io::{Error, Result};
use std::os::unix::io::AsRawFd;

/// Unlocks a file that was previously locked.
///
/// # Arguments
///
/// * `file` - A reference to the `File` that needs to be unlocked.
///
/// # Returns
///
/// This function returns a `Result` which is `Ok` if the file was successfully unlocked, or an `Err`
/// if an error occurred during unlocking.
pub fn unlock(file: &File) -> Result<()> {
    file.metadata()
        .and_then(|m| flock(file, libc::LOCK_UN, m.len() as i64))
}

/// Returns a non-blocking lock error.
///
/// This function is typically used when a non-blocking lock cannot be acquired because it is
/// already held by another process.
///
/// # Returns
///
/// Returns an `Error` corresponding to `EWOULDBLOCK`.
pub fn lock_error() -> Error {
    Error::from_raw_os_error(libc::EWOULDBLOCK)
}

/// Internal function to apply a file lock.
///
/// # Arguments
///
/// * `file` - A reference to the `File` to be locked or unlocked.
/// * `flag` - The operation to be performed, specified by constants from `libc::c_int`.
/// * `size` - File size
///
/// # Returns
///
/// Returns a `Result` which is `Ok` if the lock operation was successful, or an `Err` if an error occurred.
fn flock(file: &File, flag: libc::c_int, size: i64) -> Result<()> {
    let mut fl = libc::flock {
        l_whence: 0, // Offset from the start of the file
        l_start: 0,  // Start of the lock
        l_len: size, // Length of the lock; 0 means until EOF
        l_type: 0,   // Type of lock
        l_pid: 0,    // PID of the process holding the lock
    };

    let (cmd, operation) = match flag & libc::LOCK_NB {
        0 => (libc::F_SETLKW, flag),                 // Wait for the lock
        _ => (libc::F_SETLK, flag & !libc::LOCK_NB), // Non-blocking mode
    };

    match operation {
        libc::LOCK_SH => fl.l_type |= libc::F_RDLCK as i16,
        libc::LOCK_EX => fl.l_type |= libc::F_WRLCK as i16,
        libc::LOCK_UN => fl.l_type |= libc::F_UNLCK as i16,
        _ => return Err(Error::from_raw_os_error(libc::EINVAL)),
    }

    let ret = unsafe { libc::fcntl(file.as_raw_fd(), cmd, &fl) };
    match ret {
        -1 => match Error::last_os_error().raw_os_error() {
            Some(libc::EACCES) => return Err(lock_error()), // Handle access error as would-block error
            _ => return Err(Error::last_os_error()),
        },
        _ => Ok(()),
    }
}

/// Checks if a file is locked.
///
/// # Arguments
///
/// * `file` - A reference to the `File` to be checked.
///
/// # Returns
///
/// Returns `true` if the file is locked, `false` otherwise.
pub fn is_file_locked(file: &File) -> bool {
    file.metadata()
        .and_then(|m| is_file_locked_internal(file, m.len() as i64))
        .unwrap_or(false)
}

fn is_file_locked_internal(file: &File, size: i64) -> Result<bool> {
    let fl = libc::flock {
        l_whence: 0, // Offset from the start of the file
        l_start: 0,  // Start of the lock
        l_len: size, // Length of the lock; 0 means until EOF
        l_type: 0,   // Type of lock
        l_pid: 0,    // PID of the process holding the lock
    };

    let ret = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETLK, &fl) };
    match ret {
        -1 => match Error::last_os_error().raw_os_error() {
            Some(libc::EACCES) => return Ok(true), // Handle access error as would-block error
            _ => return Ok(false),
        },
        _ => return Ok(true),
    }
}
