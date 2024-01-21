#![allow(unused_variables)]
use anyhow::Result;

use super::{Arguments, Subcommands};
use crate::archive::BalesCompress;

impl Arguments {
    pub fn handle(self) -> Result<()> {
        match self.subcommand {
            Subcommands::Package {
                input,
                output,
                force,
                method,
            } => {
                let compress = BalesCompress::parse(input, output, force, method)?.match_type()?;
            }
        }
        Ok(())
    }
}
