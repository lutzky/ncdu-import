use crate::filetree::SizedFile;

use eyre::{eyre, Result};

/// Read a CSV file as a vector of [SizedFile]. Only the columns `path_column` and
/// `size_column` are actually read, and are used for the path and size of the file
/// respectively.
///
/// If `is_du_output` is true, then the input is expected to be the output of
/// the `du` command. In this case, `path_column` and `size_column` are ignored.
pub fn read_csv(
    path: &str,
    path_column: &str,
    size_column: &str,
    is_du_output: bool,
) -> Result<Vec<SizedFile>> {
    let mut reader = if is_du_output {
        csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path(path)?
    } else {
        csv::Reader::from_path(path)?
    };

    let mut get_col_idx = |col: &str| -> Result<usize> {
        reader
            .headers()?
            .iter()
            .position(|f| f == col)
            .ok_or_else(|| eyre!("column {col:?} missing in {path}"))
    };

    let (path_col_idx, size_col_idx) = if is_du_output {
        (1, 0)
    } else {
        (get_col_idx(path_column)?, get_col_idx(size_column)?)
    };

    reader
        .records()
        .enumerate()
        .map(|(line, record)| -> Result<SizedFile> {
            let record = record?;
            Ok(SizedFile {
                path: record
                    .get(path_col_idx)
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
