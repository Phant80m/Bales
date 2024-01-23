use super::{archive_type, BalesDecompress, BalesError, BalesUrlDecompress};
use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::{fs::File, io::copy, path::PathBuf};
use url::Url;
// parse url
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
}
// download file
impl BalesUrlDecompress {
    pub fn download(&self) -> Result<BalesDecompress, BalesError> {
        let tmp_dir = tempfile::NamedTempFile::new()
            .context("failed to create named temp file")
            .unwrap();
        let resp = ureq::builder()
            .redirects(5)
            .build()
            .get(self.input.as_str())
            .call()
            .context("failed to send request to server")
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
        copy(&mut resp.into_reader(), &mut dest.0).expect("failed to copy byes to temp dir");
        dbg!(&dest.1);
        Ok(BalesDecompress {
            input: dest.1.clone(),
            output: self.output.clone(),
            archive: archive_type(&dest.1)?,
        })
    }
}
