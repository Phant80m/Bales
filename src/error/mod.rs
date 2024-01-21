use std::path::PathBuf;
use thiserror::Error;
mod bales_error;
#[derive(Error, Debug)]
pub enum BalesError {
    NoFileExtension((String, String)),
    FileExists(PathBuf, String),
    UnknownArchiveType(String),
}
