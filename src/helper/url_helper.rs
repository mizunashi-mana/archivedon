use std::error::Error;

use url::Url;

pub struct FullUrl {
    internal: Url,
}

impl FullUrl {
    pub fn parse(url: &str) -> Result<Self, Box<dyn Error>> {
        match Url::parse(url) {
            Err(err) => Err(err.into()),
            Ok(url) => match url.domain() {
                None => Err("Any domains are not available.".into()),
                Some(_) => Ok(Self { internal: url }),
            },
        }
    }

    pub fn domain(&self) -> &str {
        match self.internal.domain() {
            None => panic!("unreachable: The domain of full URL should be available."),
            Some(x) => x,
        }
    }

    pub fn path(&self) -> &str {
        self.internal.path()
    }
}
