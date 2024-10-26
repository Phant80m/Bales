use core::panic;

use super::Updater;
use anyhow::{Context, Result};
use ewsc::{success, warning};
use owo_colors::OwoColorize;
use serde::Deserialize;
use url::Url;
use version_compare::Version;

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}
#[derive(Deserialize)]
struct Package {
    version: String,
}
impl Updater {
    pub fn is_internet() -> bool {
        use std::net::TcpStream;
        TcpStream::connect("209.85.233.101:80").is_ok()
    }
    pub fn parse(url: impl Into<String>) -> Self {
        let url = url.into();
        let url = match Url::parse(&url) {
            Ok(url) => url,
            Err(e) => panic!("failed to parse url: {}", e),
        };

        Self { url }
    }
    pub fn is_outdated(&self) -> Result<bool> {
        // get versions
        let local_version = env!("CARGO_PKG_VERSION");
        let remote_version = ureq::builder()
            .redirects(5)
            .build()
            .get(self.url.as_str())
            .call()
            .context("failed to send request to server")?;
        let remote_version: CargoToml = toml::from_str(&remote_version.into_string()?)?;
        // parse into Version,
        let local_version = Version::from(local_version);
        let remote_version = Version::from(&remote_version.package.version);
        if local_version < remote_version {
            return Ok(true);
        }
        Ok(false)
    }
    pub fn print_version(&self) -> Result<()> {
        let local_version = env!("CARGO_PKG_VERSION");
        let remote_version = ureq::builder()
            .redirects(5)
            .build()
            .get(self.url.as_str())
            .call()
            .context("failed to send request to server")?;
        let remote_version: CargoToml = toml::from_str(&remote_version.into_string()?)?;
        // parse into Version,
        let local_version = Version::from(local_version);
        let remote_version = Version::from(&remote_version.package.version);
        if local_version < remote_version {
            warning!("program is out of date:");
            warning!(
                "local version: {} < remote_version: {}",
                &local_version.unwrap(),
                &remote_version.unwrap()
            );
        } else {
            success!("program is up to date:");
            success!("bales version: {}", &local_version.unwrap())
        }
        Ok(())
    }
}
