use crate::update::Updater;
use anyhow::Result;
use core::panic;
use ewsc::warning;
use std::thread;

use super::{Arguments, Subcommands};
use crate::archive::{BalesCompress, BalesDecompress};

impl Arguments {
    pub fn handle(self) -> Result<()> {
        // let up_to_date =
        let thread = thread::spawn(move || {
            if Updater::is_internet() {
                if Updater::parse(
                    "https://raw.githubusercontent.com/Phant80m/bales/main/Cargo.toml",
                )
                .is_outdated()
                .unwrap()
                {
                    use owo_colors::OwoColorize;
                    warning!("program out of date! Please update to the latest version");
                    warning!(
                        "if you do not want to show this warning add the flag --ignore-updates"
                    );
                }
            }
        });

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
            None => panic!("Unknown command"),
        }
        thread.join().unwrap();
        Ok(())
    }
}
