mod tar;
mod url_decompress;
mod utils;
mod zip;
use crate::error::BalesError;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::copy;
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
impl BalesUrlDecompress {
    pub fn download(&self) -> Result<BalesDecompress, BalesError> {
        let tmp_dir = tempfile::NamedTempFile::new()
            .context("failed to create named temp file")
            .unwrap();

        let response = reqwest::blocking::get(self.input.as_str())
            .context("failed to get response")
            .unwrap();
        let mut dest: (File, PathBuf) = {
            // println!("file to download: '{}'", fname);
            let extension = &PathBuf::from(&self.input.to_string())
                .extension()
                .expect("a file extension present")
                .to_string_lossy()
                .to_string();
            // let fname = tmp_dir.with_extension(extension)
            let fname = tmp_dir.path().with_extension(extension);
            println!("will be located under: '{:?}'", fname);
            (
                File::create(&fname).expect("failed to create tempdir"),
                fname,
            )
        };
        let content = response.text().expect("failed to get response text");
        copy(&mut content.as_bytes(), &mut dest.0).expect("failed to copy byes to temp dir");
        dbg!(&dest.1);
        Ok(BalesDecompress {
            input: dest.1.clone(),
            output: self.output.clone(),
            archive: archive_type(&dest.1)?,
        })
    }
}
impl BalesDecompress {
    pub fn parse_url(
        input: PathBuf,
        output: Option<PathBuf>,
    ) -> Result<BalesUrlDecompress, BalesError> {
        // valid url checks
        if output.is_none() {
            return Err(BalesError::NoOutputSpecified(input.display().to_string()));
        }
        let input = input.display().to_string();

        let url = Url::parse(&input);
        if url.is_err() {
            if let Err(err) = url {
                if err == url::ParseError::RelativeUrlWithoutBase {
                    return Err(BalesError::RelativeUrlWithoutBase(input));
                } else if err == url::ParseError::InvalidDomainCharacter {
                    return Err(BalesError::InvalidChar(input));
                }
                println!("{}", err);
            }
        }
        let output = output.unwrap();
        Ok(BalesUrlDecompress {
            input: url.clone().unwrap(),
            output: output.clone(),
            archive: archive_type(&PathBuf::from(&url.unwrap().as_str()))?,
        })
    }
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
fn archive_type(path: &PathBuf) -> Result<Archive, BalesError> {
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
