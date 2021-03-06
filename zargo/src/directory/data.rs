//!
//! The circuit `data` directory.
//!

use std::fs;
use std::io;
use std::path::PathBuf;

use failure::Fail;

pub struct Directory {}

static DIRECTORY_NAME_DEFAULT: &str = "data/";

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),
}

impl Directory {
    pub fn create(path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }

        fs::create_dir_all(&path).map_err(Error::Creating)
    }

    pub fn remove(path: &PathBuf) -> Result<(), Error> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(DIRECTORY_NAME_DEFAULT) {
            path.push(PathBuf::from(DIRECTORY_NAME_DEFAULT));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(Error::Removing)?;
        }

        Ok(())
    }
}
