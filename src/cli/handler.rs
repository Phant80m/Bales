use crate::{error::BalesError, update::Updater};
use anyhow::Result;
use core::panic;
use ewsc::warning;
use owo_colors::OwoColorize;

use super::{Arguments, Subcommands};
use crate::archive::{BalesCompress, BalesDecompress};
const PKG_URL: &str = "https://raw.githubusercontent.com/Phant80m/bales/main/Cargo.toml";
impl Arguments {
    pub fn handle(self) -> Result<()> {
        if self.version {
            Updater::parse(PKG_URL).print_version().unwrap();
            std::process::exit(0)
        }

        match self.subcommand {
            Some(Subcommands::Package {
                input,
                output,
                force,
            }) => {
                BalesCompress::parse(input, output, force)?.match_type()?;
            }
            Some(Subcommands::UnPackage { url, input, output }) => {
                if !url {
                    BalesDecompress::parse(input, output)?.match_type()?;
                } else {
                    BalesDecompress::parse_url(input, output)?
                        .download()?
                        .match_type()?;
                }
            }
            None => {
                eprintln!(
                    "{}\n{}{}\n{}{}{}",
                    "Error:",
                    "󱞪  ".bold(),
                    "Unknown command:".red().bold(),
                    "   run",
                    " --help ".green(),
                    "to list available commands"
                );
                return Ok(());
            }
        }
        if Updater::is_internet() && Updater::parse(PKG_URL).is_outdated().unwrap() {
            warning!("program out of date! Please update to the latest version");
        }
        Ok(())
    }
}
