use archivedon::redirect_map::RedirectMap;
use archivedon::resource_path::ResourcePath;
use archivedon::webfinger::resource::Resource as WebfingerResource;
use serde::Serialize;
use std::{error::Error, path::PathBuf};
use tokio::fs;

pub struct Output {
    resource_path: ResourcePath,
}

impl Output {
    pub async fn create(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        if !fs::try_exists(&path).await? {
            fs::create_dir_all(&path).await?;
        }

        Ok(Self {
            resource_path: ResourcePath::new(path),
        })
    }

    pub async fn save_webfinger_resource(
        &self,
        content: &WebfingerResource,
    ) -> Result<(), Box<dyn Error>> {
        let save_path = self.resource_path.webfinger_path(&content.subject);
        fs::write(&save_path, serde_json::to_vec(content)?).await?;
        Ok(())
    }

    pub async fn save_static_json_resource<T: Serialize>(
        &self,
        path: &str,
        content: &T,
    ) -> Result<(), Box<dyn Error>> {
        let save_path = self.resource_path.static_root_dir.join(path);
        fs::create_dir_all(save_path.parent().unwrap()).await?;
        fs::write(&save_path, serde_json::to_vec(content)?).await?;
        Ok(())
    }

    pub async fn save_static_text_resource(
        &self,
        path: &str,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        let save_path = self.resource_path.static_root_dir.join(path);
        fs::create_dir_all(save_path.parent().unwrap()).await?;
        fs::write(&save_path, content).await?;
        Ok(())
    }

    pub async fn get_redirect_map_resource(
        &self,
        domain: &str,
        url_path: &str,
    ) -> Result<Option<RedirectMap>, Box<dyn Error>> {
        let save_path = self.resource_path.redirect_map_path(domain, url_path);
        if fs::try_exists(&save_path).await? {
            let resource = fs::read(&save_path).await?;
            match serde_json::from_slice(&resource) {
                Ok(resource) => Ok(Some(resource)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn save_redirect_map_resource(
        &self,
        domain: &str,
        url_path: &str,
        resource: &RedirectMap,
    ) -> Result<(), Box<dyn Error>> {
        let save_path = self.resource_path.redirect_map_path(domain, url_path);
        fs::create_dir_all(save_path.parent().unwrap()).await?;
        fs::write(&save_path, serde_json::to_vec(resource)?).await?;
        Ok(())
    }
}
