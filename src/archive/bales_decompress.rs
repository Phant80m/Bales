use super::{archive_type, Archive, BalesDecompress, BalesError};
use anyhow::Result;
use std::path::PathBuf;

impl BalesDecompress {
    pub fn parse(input: PathBuf, output: Option<PathBuf>) -> Result<Self, BalesError> {
        // check input validity

        // generate assumed output for example input = foo.tar.gz -> output: ./foo/

        let file_stem_str = input
            .file_stem()
            .expect("file has no name!")
            .to_str()
            .unwrap()
            .to_string();
        let file_stem = if file_stem_str.ends_with(".tar") {
            &file_stem_str[..file_stem_str.len() - 4]
        } else {
            &file_stem_str
        };
        let output = output.unwrap_or(PathBuf::from(file_stem));
        // handle error for no file extension on input
        let i_extension = input.extension();
        if i_extension.is_none() {
            return Err(BalesError::NoFileExtension((
                input.display().to_string(),
                output.display().to_string(),
                "extract".to_string(),
            )));
        }
        // make sure input exists!
        if !input.exists() {
            return Err(BalesError::NoFileExists(input, "archive".to_string()));
        }

        Ok(Self {
            input: input.clone(),
            output,
            archive: archive_type(&input)?,
        })
    }
    pub fn match_type(&self) -> Result<()> {
        match &self.archive {
            Archive::Tar => self.from_tar()?,
            Archive::Zip => self.from_zip()?,
        };
        Ok(())
    }
}
