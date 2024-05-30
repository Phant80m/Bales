mod bales_compress;
mod bales_decompress;
mod compression;
mod url_decompress;
mod utils;
use crate::error::BalesError;
use anyhow::{Context, Result};
use std::path::Path;
use std::path::PathBuf;
use strum_macros::{EnumString, EnumVariantNames};
use url::Url;
// compress
pub struct BalesCompress {
    pub input: Vec<PathBuf>,
    pub output: PathBuf,
    pub archive: Archive,
}

#[derive(Debug, EnumString, EnumVariantNames)]
pub enum Archive {
    #[strum(serialize = ".tar.gz")]
    Tar,
    #[strum(serialize = ".zip")]
    Zip,
}

// decompress
#[derive(Debug)]
pub struct BalesDecompress {
    pub input: PathBuf,
    pub output: PathBuf,
    pub archive: Archive,
}
pub struct BalesUrlDecompress {
    pub input: Url,
    pub output: PathBuf,
    pub archive: Archive,
}
fn archive_type(path: &Path) -> Result<Archive, BalesError> {
    let extension = path
        .extension()
        .context("failed to find file extension")
        .unwrap()
        .to_string_lossy()
        .to_string()
        .to_lowercase();
    match extension.as_str() {
        "gz" => Ok(Archive::Tar),
        "zip" => Ok(Archive::Zip),
        _ => Err(BalesError::UnknownArchiveType(extension)),
    }
}
