use std::{path::Path, sync::Arc};

use archivedon::resource_path::ResourcePath;
use url::Url;

pub mod user;

pub const PROG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROG_REPOSITORY: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");
pub const PROG_HOMEPAGE: Option<&str> = option_env!("CARGO_PKG_HOMEPAGE");

pub struct Env {
    pub resource_path: ResourcePath,
    pub expose_url_base: Url,
}

impl Env {
    pub fn load(resource_dir: &Path, expose_url_base: Url) -> Arc<Env> {
        Arc::new(Env {
            resource_path: ResourcePath::new(resource_dir.to_path_buf()),
            expose_url_base,
        })
    }
}
