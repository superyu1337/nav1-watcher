use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check a directory for changes
    Check(CheckArgs),
    Datadir,
    Cleardata,
}

#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Configuration file to use
    #[arg(short, long)]
    pub config: PathBuf,

    /// Dont fire discord webhook, only persist directories.
    #[arg(short, long)]
    pub no_webhook: bool,
}