use super::BalesCompress;
use anyhow::Result;
use ewsc::success;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, io, path::Path};
use term_size::dimensions;
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

// misc functions
pub fn total_files(path: &Path) -> usize {
    WalkDir::new(path).into_iter().count()
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

pub fn custom_format(bar_size: usize) -> String {
    format!(
        "[{{elapsed_precise}}{{spinner}}] [{{bar:{bar_size}.yellow/white}}] {{pos:>7.yellow}}/{{len:7}} {{msg}}",
        bar_size = bar_size
    )
}

pub fn term_size() -> usize {
    let dims = dimensions().unwrap_or((45, 0)).0;
    dims
}
