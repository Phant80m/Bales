mod tar;
mod zip;
use crate::error::BalesError;
use anyhow::Result;
use std::path::PathBuf;
use strum_macros::{EnumString, EnumVariantNames};
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
impl BalesCompress {
    pub fn parse(
        mut input: Vec<PathBuf>,
        output: PathBuf,
        force: bool,
        method: Option<String>,
    ) -> Result<Self, BalesError> {
        // check path validity
        input.retain(|path| {
            if !path.exists() {
                ewsc::warning!(
                    "path: '{}' Does not exist proceeding without it!",
                    path.display()
                );
            }
            path.exists()
        });

        let input_string: String = input
            .iter()
            .map(|path| path.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        if output.extension().is_none() {
            return Err(BalesError::NoFileExtension((
                input_string,
                output.display().to_string(),
            )));
        }
        // make sure output path does not exist
        if !force {
            if output.exists() {
                return Err(BalesError::FileExists(output, "output".to_string()));
            }
        }

        Ok(Self {
            input,
            output: output.clone(),
            archive: archive_type(&output)?,
        })
    }
    pub fn match_type(&self) -> Result<()> {
        match &self.archive {
            Archive::Tar => self.into_tar()?,
            Archive::Zip => self.into_zip()?,
        };
        Ok(())
    }
}
fn archive_type(path: &PathBuf) -> Result<Archive, BalesError> {
    let extension = path
        .extension()
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
