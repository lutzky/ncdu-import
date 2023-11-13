mod filetree;
mod import;
mod ncdu;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Input CSV file
    input: PathBuf,

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

use eyre::eyre;
use eyre::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input_csv = import::read_csv(
        cli.input
            .to_str()
            .ok_or(eyre!("invalid input path {:?}", cli.input))?,
        &cli.path_column,
        &cli.size_column,
        cli.is_du_output,
    )?;

    let tree = filetree::Tree::from(input_csv);
    let export: ncdu::Export = tree.into();
    let json = serde_json::to_string_pretty(&export)?;
    println!("{json}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use goldenfile::Mint;
    use std::io::Write;

    #[rstest::rstest]
    fn golden_test(#[files("testdata/*.csv")] input_file: PathBuf) {
        let mut mint = Mint::new(input_file.parent().unwrap());

        let want_output = input_file.with_extension("json");

        let mut golden_simple = mint
            .new_goldenfile(want_output.file_name().unwrap())
            .unwrap();

        let input_csv =
            import::read_csv(input_file.to_str().unwrap(), "name", "size", false).unwrap();

        let tree = filetree::Tree::from(input_csv);
        let export: ncdu::Export = tree.into();
        let json = serde_json::to_string_pretty(&export).unwrap();

        write!(golden_simple, "{json}").unwrap();
    }
}
