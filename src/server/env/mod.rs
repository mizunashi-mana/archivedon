use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub mod user;

pub struct Env {
    pub resource_dir: PathBuf,
}

impl Env {
    pub fn load(resource_dir: &Path) -> Arc<Env> {
        Arc::new(Env {
            resource_dir: resource_dir.to_path_buf(),
        })
    }
}
