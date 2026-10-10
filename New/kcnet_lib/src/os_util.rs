// use std::error::Error;
// use homedir::my_home;
use homedir::home;
use std::{io, path::PathBuf};
// https://doc.rust-lang.org/std/env/fn.home_dir.html
// https://docs.rs/homedir/latest/homedir/

/// Get the users home directory.
fn home_dir(username: &str) -> Result<Option<PathBuf>, homedir::GetHomeError>   {
    home(username)
}

/// Get the users home directory, directly as a string and catches errors.
pub fn get_home_dir(username: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = home_dir(username)?
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            format!("No home directory found for user {username:?}"),
        ))?;

    // https://stackoverflow.com/questions/64996954/how-can-i-pull-data-out-of-an-option-for-independent-use
    // https://oneuptime.com/blog/post/2026-02-20-rust-error-handling-result/view

    // Strip the 'Some' and 'Ok' values from this.
    let home_dir = path;
    Ok(home_dir)
}