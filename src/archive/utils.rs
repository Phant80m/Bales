use std::path::Path;
use term_size::dimensions;
use walkdir::WalkDir;
// misc functions
pub fn total_files(path: &Path) -> usize {
    WalkDir::new(path).into_iter().count()
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
