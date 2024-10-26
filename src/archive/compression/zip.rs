use crate::archive::utils::*;
use owo_colors::OwoColorize;
use zip::ZipArchive;

use crate::archive::{BalesCompress, BalesDecompress};
use anyhow::Context;
use anyhow::Result;
use ewsc::success;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Cursor;
use std::{fs::File, io, path::Path};
use walkdir::WalkDir;
use zip::write::FileOptions;

impl BalesCompress {
    pub fn into_zip(&self) -> Result<()> {
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
        // zip
        let zip_file = File::create(&self.output)?;
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);

        let mut zip = zip::write::ZipWriter::new(zip_file);

        for items in &self.input {
            let path = Path::new(items);
            if path.is_file() {
                add_files_to_zip(&mut zip, path, options)?;
                bar.inc(1);
            } else if path.is_dir() {
                for entry in WalkDir::new(items) {
                    let entry = entry?;
                    let path = entry.path();
                    add_files_to_zip(&mut zip, path, options)?;
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

fn add_files_to_zip<W>(
    zip: &mut zip::write::ZipWriter<W>,
    file_path: &Path,
    options: FileOptions,
) -> Result<(), io::Error>
where
    W: io::Write + io::Seek,
{
    if file_path.is_file() {
        let relative_path = file_path.strip_prefix(".").unwrap_or(file_path);
        zip.start_file(relative_path.to_str().unwrap(), options)?;
        let mut file = File::open(file_path)?;
        io::copy(&mut file, zip)?;
    }
    Ok(())
}

// decompress
impl BalesDecompress {
    pub fn from_zip(&self) -> Result<()> {
        let input = &self
            .input
            .canonicalize()
            .expect("failed to canonicalize path");
        //
        let archive = std::fs::read(input)
            .context("error reading zip file contents")
            .expect("failed to read zip archive");

        let mut zip = ZipArchive::new(Cursor::new(archive))?;
        let total = zip.len();
        // create bar
        let bar = ProgressBar::new(total as u64);
        bar.set_style(
            ProgressStyle::with_template(&custom_format(term_size() - ((term_size() * 2) / 3) + 6))
                .unwrap()
                .progress_chars("=> "),
        );

        for i in 0..zip.len() {
            bar.inc(1);
            let mut file = zip.by_index(i).unwrap();
            bar.set_message(format!("\n extracting: {}", file.name()));

            let output_path = Path::new(&self.output);

            if file.name().ends_with('/') {
                std::fs::create_dir_all(output_path.join(file.name()))?;
            } else {
                let mut output_file = File::create(output_path.join(file.name()))?;

                std::io::copy(&mut file, &mut output_file)?;
            }
        }
        // // zip_extract::extract(Cursor::new(archive), &self.output, false)
        //     .context("failed to extract contents of zip")?;
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
