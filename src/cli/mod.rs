mod base64_opts;
mod csv_opts;
mod http;
mod passgen_opts;
mod text;
pub use base64_opts::{Base64Format, Base64SubCommand};
use clap::Parser;
use csv_opts::CsvOpts;
pub use http::HttpSubCommand;
use passgen_opts::PassgenOpts;
use std::path::{Path, PathBuf};

pub use csv_opts::OutputFormat;
pub use text::{TextSignFormat, TextSubCommand};

use crate::CmdExecuter;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "passgen", about = "Generate a random password")]
    Passgen(PassgenOpts),
    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign or verify text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP Server")]
    Http(HttpSubCommand),
}

impl CmdExecuter for SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::Passgen(opts) => opts.execute().await,
            SubCommand::Base64(opts) => opts.execute().await,
            SubCommand::Text(opts) => opts.execute().await,
            SubCommand::Http(opts) => opts.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("non-exist"), Err("File does not exist"));
    }
}
