use archivedon::webfinger::resource::Resource as WebfingerResource;
use serde::Serialize;
use std::{error::Error, path::PathBuf};
use tokio::fs;

pub struct Output {
    static_dir: PathBuf,
    webfinger_resource_dir: PathBuf,
}

impl Output {
    pub async fn create(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        if !fs::try_exists(&path).await? {
            fs::create_dir_all(&path).await?;
        }

        let static_dir = path.join("static");
        if !fs::try_exists(&static_dir).await? {
            fs::create_dir(&static_dir).await?;
        }

        let webfinger_resource_dir = path.join("webfinger");
        if !fs::try_exists(&webfinger_resource_dir).await? {
            fs::create_dir(&webfinger_resource_dir).await?;
        }

        Ok(Self {
            static_dir,
            webfinger_resource_dir,
        })
    }

    pub async fn save_webfinger_resource(
        &self,
        content: &WebfingerResource,
    ) -> Result<(), Box<dyn Error>> {
        let filename = format!("{}.json", content.subject);
        let save_path = self.webfinger_resource_dir.join(&filename);
        fs::write(&save_path, serde_json::to_vec(content)?).await?;
        Ok(())
    }

    pub async fn save_static_json_resource<T: Serialize>(
        &self,
        path: &str,
        content: &T,
    ) -> Result<(), Box<dyn Error>> {
        let save_path = self.static_dir.join(path);
        fs::create_dir_all(save_path.parent().unwrap()).await?;
        fs::write(&save_path, serde_json::to_vec(content)?).await?;
        Ok(())
    }

    pub async fn save_static_text_resource(
        &self,
        path: &str,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        let save_path = self.static_dir.join(path);
        fs::create_dir_all(save_path.parent().unwrap()).await?;
        fs::write(&save_path, content).await?;
        Ok(())
    }
}
