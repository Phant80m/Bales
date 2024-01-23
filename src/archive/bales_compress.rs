use super::{archive_type, Archive, BalesCompress, BalesError};
use anyhow::Result;
use std::path::PathBuf;

impl BalesCompress {
    pub fn parse(
        mut input: Vec<PathBuf>,
        output: PathBuf,
        force: bool,
    ) -> Result<Self, BalesError> {
        // check output extension
        if output.extension().is_none() {
            return Err(BalesError::NoFileExtension((
                input
                    .iter()
                    .map(|path| path.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" "),
                output.display().to_string(),
                "compress".to_string(),
            )));
        }
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
