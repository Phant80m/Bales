use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod handler;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    /// compress files / dir into a tar / zip
    #[clap(name = "compress")]
    Package {
        input: Vec<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        // force override output
        force: bool,
    },
    /// extract the contents of a tar / zip
    #[clap(name = "extract")]
    UnPackage {
        #[arg(long, short)]
        url: bool,
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
