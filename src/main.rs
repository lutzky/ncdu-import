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

mod filetree;
mod import;
mod ncdu;

use clap::Parser;
use std::{
    fs::File,
    io::{stdin, Read},
};

#[derive(Parser)]
struct Cli {
    /// Input CSV file (- for standard input)
    input: String,

    /// Output JSON file (- for standard output)
    #[arg(short, long)]
    output: String,

    /// This column should be read for file paths
    #[arg(long, default_value = "name")]
    path_column: String,

    /// This column should be read for file sizes
    #[arg(long, default_value = "size")]
    size_column: String,

    /// If true, instead of CSV format, the input is expected to be as outputted
    /// from the `du -a -b` command.
    #[arg(long, default_value = "false")]
    is_du_output: bool,
}

use eyre::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input: Box<dyn Read> = if cli.input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(cli.input)?)
    };

    let sized_file_vec =
        import::read_csv(input, &cli.path_column, &cli.size_column, cli.is_du_output)?;

    let tree = filetree::Tree::from(sized_file_vec);
    let export: ncdu::Export = tree.into();
    let json = serde_json::to_string_pretty(&export)?;

    if cli.output == "-" {
        println!("{json}");
    } else {
        std::fs::write(cli.output, json)?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use goldenfile::Mint;
    use std::{io::Write, path::PathBuf};

    #[rstest::rstest]
    fn golden_test(#[files("testdata/*.csv")] input_file: PathBuf) {
        let mut mint = Mint::new(input_file.parent().unwrap());

        let want_output = input_file.with_extension("json");

        let mut golden_simple = mint
            .new_goldenfile(want_output.file_name().unwrap())
            .unwrap();

        let input_csv =
            import::read_csv(File::open(input_file).unwrap(), "name", "size", false).unwrap();

        let tree = filetree::Tree::from(input_csv);
        let export: ncdu::Export = tree.into();
        let json = serde_json::to_string_pretty(&export).unwrap();

        write!(golden_simple, "{json}").unwrap();
    }
}
