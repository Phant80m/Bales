use crate::archive::utils::*;
use crate::archive::{BalesCompress, BalesDecompress};
use anyhow::{Context, Result};
use ewsc::success;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::fs;
use std::io::Seek;
use std::{fs::File, path::Path};
use tar::Archive;
use walkdir::WalkDir;

impl BalesCompress {
    pub fn into_tar(&self) -> Result<()> {
        // progress bar
        let total_files = self
            .input
            .iter()
            .map(|path| total_files(path))
            .sum::<usize>();
        let bar = ProgressBar::new(total_files as u64);

        bar.set_style(
            ProgressStyle::with_template(&custom_format(term_size() - ((term_size() * 2) / 3) + 6))
                .unwrap()
                .progress_chars("=> "),
        );

        let tarball = File::create(&self.output).expect("Failed to create tarball");
        let enc = GzEncoder::new(tarball, Compression::fast());
        let mut tar = tar::Builder::new(enc);
        for items in &self.input {
            let path = Path::new(items);
            if path.is_file() {
                let mut file = File::open(path).context("failed to open file")?;
                tar.append_file(path, &mut file)?;
                bar.inc(1);
            } else if path.is_dir() {
                for entry in WalkDir::new(items) {
                    let entry = entry?;
                    let path = entry.path();
                    let mut file = File::open(path).context("failed to open file")?;
                    if file.metadata()?.is_file() {
                        tar.append_file(path, &mut file)
                            .context("failed to add file to tar")?;
                    }
                    bar.inc(1);
                }
            }
        }
        bar.finish();
        success!(
            "Created zip archive at: {}",
            &self
                .output
                .canonicalize()
                .expect("failed to canoncalize path")
                .display()
                .underline()
                .green()
        );

        Ok(())
    }
}

impl BalesDecompress {
    pub fn from_tar(&self) -> Result<()> {
        let input = &self.input;
        let mut tar_gz = File::open(input)?;
        let tar = GzDecoder::new(&tar_gz);
        let mut archive = Archive::new(tar);
        // if output not exists!
        if !&self.output.exists() {
            fs::create_dir(&self.output).context("failed to create output")?;
        }
        // total archive
        let total = archive.entries()?.count();
        println!("{}", total);
        // create bar
        let bar = ProgressBar::new(total as u64);

        bar.set_style(
            ProgressStyle::with_template(&custom_format(term_size() - ((term_size() * 2) / 3) + 6))
                .unwrap()
                .progress_chars("=> "),
        );
        // set file to begining
        tar_gz.seek(std::io::SeekFrom::Start(0))?;
        // re init archive
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        //
        for entry in archive.entries()? {
            let mut entry = entry?;
            // set bar message as
            bar.set_message(format!(
                "\n extracting: {}",
                &entry.path()?.file_name().unwrap().to_str().unwrap()
            ));
            bar.inc(1);
            if !entry.unpack_in(&self.output).context("LINE: 73")? {
                panic!("error: sketchy tar tried to unpack outside its root.")
            }
        }
        bar.finish();
        success!(
            "Unpacked archive at: {}",
            &self
                .output
                .canonicalize()
                .expect("failed to canoncalize path")
                .display()
                .underline()
                .green()
        );

        Ok(())
    }
}
