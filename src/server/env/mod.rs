use std::{path::Path, sync::Arc};

use archivedon::resource_path::ResourcePath;

pub mod user;

pub struct Env {
    pub resource_path: ResourcePath,
}

impl Env {
    pub fn load(resource_dir: &Path) -> Arc<Env> {
        Arc::new(Env {
            resource_path: ResourcePath::new(resource_dir.to_path_buf()),
        })
    }
}
