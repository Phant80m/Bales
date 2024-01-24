use super::{archive_type, BalesDecompress, BalesError, BalesUrlDecompress};
use crate::archive::utils::*;
use anyhow::{Context, Result};
use ewsc::error;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::io::{Cursor, Read};
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
        let output = output.unwrap();
        if !output.exists() {
            match std::fs::create_dir(&output) {
                Ok(dir) => dir,
                Err(e) => {
                    error!("failed to create output directory: {}", e);
                }
            }
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
        Ok(BalesUrlDecompress {
            input: url.clone().unwrap(),
            output: output.clone(),
            archive: archive_type(&PathBuf::from(&url.unwrap().as_str()))?,
        })
    }
}
// download file
const CHUNK_SIZE: usize = 10;
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
        // bar
        let expected_len = match resp.header("Content-Length") {
            Some(hdr) => hdr.parse().expect("can't parse number"),
            None => 100,
        };
        let mut buf_len = 0usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(expected_len);
        let mut reader = resp.into_reader();
        let bar = ProgressBar::new(expected_len as u64);
        bar.set_message("Downloading");
        bar.set_style(
            ProgressStyle::with_template(&custom_dl_format(
                term_size() - ((term_size() * 2) / 3) + 6,
            ))
            .unwrap()
            .progress_chars("=> "),
        );
        bar.set_message(format!("Downloading: {}", &self.input.as_str()));

        // response
        let mut dest: (File, PathBuf) = {
            let extension = &PathBuf::from(&self.input.to_string())
                .extension()
                .expect("a file extension present")
                .to_string_lossy()
                .to_string();
            let fname = tmp_dir.path().with_extension(extension);
            loop {
                // Grow our buffer, read to it, and store the number of written bytes
                // Note that we won't always read exactly CHUNK_SIZE bytes, so sometimes we
                buffer.extend_from_slice(&[0; CHUNK_SIZE]);
                let chunk = &mut buffer.as_mut_slice()[buf_len..buf_len + CHUNK_SIZE];
                let read_bytes = reader.read(chunk).expect("error reading stream");
                buf_len += read_bytes;

                bar.set_position(buf_len as u64);

                // Break if our stream is empty
                if read_bytes == 0 {
                    break;
                }
            }
            (
                File::create(&fname).expect("failed to create tempdir"),
                fname,
            )
        };
        buffer.truncate(buf_len);
        copy(&mut Cursor::new(buffer), &mut dest.0).expect("failed to copy byes to temp dir");
        if !&dest.1.exists() {
            return Err(BalesError::NoFileExists(
                dest.1,
                "Downloaded file".to_string(),
            ));
        }
        Ok(BalesDecompress {
            input: dest.1.clone(),
            output: self.output.clone(),
            archive: archive_type(&dest.1)?,
        })
    }
}
