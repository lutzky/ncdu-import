use crate::filetree::SizedFile;

use eyre::{eyre, Result};

/// Read a CSV file as a vector of [SizedFile]. Only the columns `name_col` and
/// `size_col` are actually read, and are used for the path and size of the file
/// respectively.
pub fn read_csv(path: &str, name_col: &str, size_col: &str) -> Result<Vec<SizedFile>> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut get_col_idx = |col: &str| -> Result<usize> {
     reader
        .headers()?
        .iter()
        .position(|f| f == col)
        .ok_or_else(|| eyre!("column {col:?} missing in {path}"))
    };

    let name_col_idx = get_col_idx(name_col)?;
    let size_col_idx = get_col_idx(size_col)?;

    reader
        .records()
        .enumerate()
        .map(|(line, record)| -> Result<SizedFile> {
            let record = record?;
            Ok(SizedFile {
                path: record
                    .get(name_col_idx)
                    .ok_or_else(|| eyre!("line {line} of {path} is missing columns"))?
                    .to_owned(),
                size: record
                    .get(size_col_idx)
                    .ok_or_else(|| eyre!("line {line} of {path} is missing columns"))?
                    .parse()?,
            })
        })
        .collect()
}
