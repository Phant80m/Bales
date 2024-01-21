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
    #[clap(name = "pkg")]
    Package {
        input: Vec<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        // force override output
        force: bool,
    },
    #[clap(name = "upkg")]
    UnPackage {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
