use url::Url;

use super::Updater;

impl Updater {
    fn parse(url: impl Into<String>) -> Self {
        let url = url.into();
        let url = match Url::parse(&url) {
            Ok(url) => url,
            Err(e) => panic!("failed to parse url"),
        };

        Self { url }
    }
    pub fn check_for_update(&self) {
        let local_version = env!("CARGO_PKG_VERSION");
    }
}
