#![allow(unused_variables)]
use anyhow::Result;

use super::{Arguments, Subcommands};
use crate::archive::{BalesCompress, BalesDecompress};

impl Arguments {
    pub fn handle(self) -> Result<()> {
        match self.subcommand {
            Subcommands::Package {
                input,
                output,
                force,
            } => {
                let compress = BalesCompress::parse(input, output, force)?.match_type()?;
            }
            Subcommands::UnPackage { url, input, output } => {
                if !url {
                    let decompress = BalesDecompress::parse(input, output)?.match_type()?;
                } else {
                    BalesDecompress::parse_url(input, output)?
                        .download()?
                        .match_type()?;
                }
            }
        }
        Ok(())
    }
}
