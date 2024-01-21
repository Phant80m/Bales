use super::BalesCompress;
use crate::archive::zip::{custom_format, term_size, total_files};
use anyhow::{Context, Result};
use ewsc::success;
use flate2::{write::GzEncoder, Compression};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, path::Path};
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
