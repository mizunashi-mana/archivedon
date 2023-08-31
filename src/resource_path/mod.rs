use std::path::PathBuf;

use crate::helper::path_helper;

pub struct ResourcePath {
    pub index_html_path: PathBuf,
    webfinger_root_dir: PathBuf,
    pub static_root_dir: PathBuf,
    redirect_map_root_dir: PathBuf,
}

impl ResourcePath {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            index_html_path: root_dir.join("index.html"),
            webfinger_root_dir: root_dir.join("webfinger"),
            static_root_dir: root_dir.join("static"),
            redirect_map_root_dir: root_dir.join("map"),
        }
    }

    pub fn webfinger_path(&self, subject: &str) -> PathBuf {
        self.webfinger_root_dir.join(format!(
            "{}.json",
            path_helper::component_to_url_safe(subject)
        ))
    }

    pub fn redirect_map_path(&self, domain: &str, url_path: &str) -> PathBuf {
        path_helper::to_url_safe(&self.redirect_map_root_dir, domain, url_path, "json")
    }
}
