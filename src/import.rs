// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::filetree::SizedFile;

use eyre::{eyre, Result};

/// Read a CSV file as a vector of [SizedFile]. Only the columns `path_column` and
/// `size_column` are actually read, and are used for the path and size of the file
/// respectively.
///
/// If path is "-", standard input is used.
///
/// If `is_du_output` is true, then the input is expected to be the output of
/// the `du` command. In this case, `path_column` and `size_column` are ignored.
pub fn read_csv(
    input: impl std::io::Read,
    path_column: &str,
    size_column: &str,
    is_du_output: bool,
) -> Result<Vec<SizedFile>> {
    let mut reader_builder = csv::ReaderBuilder::new();

    let mut reader = if is_du_output {
        reader_builder.delimiter(b'\t').has_headers(false)
    } else {
        reader_builder.has_headers(true)
    }
    .from_reader(input);

    let mut get_col_idx = |col: &str| -> Result<usize> {
        reader
            .headers()?
            .iter()
            .position(|f| f == col)
            .ok_or_else(|| eyre!("column {col:?} missing"))
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
                    .ok_or_else(|| eyre!("line {line} is missing columns"))?
                    .to_owned(),
                size: record
                    .get(size_col_idx)
                    .ok_or_else(|| eyre!("line {line} is missing columns"))?
                    .parse()?,
            })
        })
        .collect()
}
