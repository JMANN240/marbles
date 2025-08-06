use std::{error::Error, thread, time::Duration};

use reqwest::blocking::Client;
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct MediaResponse {
    id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerStatusResponse {
    status_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaPublishResponse {
    id: String,
}

pub struct InstagramPoster {
    client: Client,
    app_scoped_user_id: String,
    user_access_token: String,
}

impl InstagramPoster {
    pub fn new(app_scoped_user_id: String, user_access_token: String) -> Self {
        Self {
            client: Client::new(),
            app_scoped_user_id,
            user_access_token,
        }
    }

    pub async fn post(
        &self,
        caption: &str,
        video_url: &str,
        thumb_offset: f64,
    ) -> Result<MediaPublishResponse, Box<dyn Error>> {
        let media_response = self.create_media(caption, video_url, thumb_offset)?;

        info!("Sleeping...");
        thread::sleep(Duration::from_secs(10));

        loop {
            match self.get_container_status(&media_response.id) {
                Ok(container_status_response) => {
                    if container_status_response.status_code == "FINISHED" {
                        return self.publish_media(&media_response.id);
                    }
                },
                Err(e) => {
                    info!(?e, "Checking Container Status Failed!");
                    return Err(e);
                }
            }

            info!("Sleeping...");
            thread::sleep(Duration::from_secs(10));
        }
    }

    pub fn create_media(&self, caption: &str, video_url: &str, thumb_offset: f64) -> Result<MediaResponse, Box<dyn Error>> {
        info!("Creating Instagram Media...");

        let media_response = self.client
            .post(format!(
                "https://graph.instagram.com/v23.0/{}/media",
                &self.app_scoped_user_id
            ))
            .form(&[
                ("media_type", "REELS"),
                ("video_url", video_url),
                ("caption", caption),
                ("share_to_feed", "true"),
                ("thumb_offset", &thumb_offset.to_string()),
                ("access_token", &self.user_access_token),
            ])
            .send()?
            .text()?;

        info!(?media_response, "Instagram Media Created!");

        Ok(serde_json::from_str(&media_response)?)
    }

    pub fn get_container_status(&self, container_id: &str) -> Result<ContainerStatusResponse, Box<dyn Error>> {
        info!("Checking Container Status...");

        let container_status_response = self.client
            .get(format!(
                "https://graph.instagram.com/v23.0/{}",
                container_id
            ))
            .query(&[
                ("fields", "status_code,status"),
                ("access_token", &self.user_access_token),
            ])
            .send()?
            .text()?;

        info!(?container_status_response, "Checked Container Status!");

        Ok(serde_json::from_str(&container_status_response)?)
    }

    pub fn publish_media(&self, creation_id: &str) -> Result<MediaPublishResponse, Box<dyn Error>> {
        info!("Publishing Instagram Media...");

        let media_publish_response = self
            .client
            .post(format!(
                "https://graph.instagram.com/v23.0/{}/media_publish",
                &self.app_scoped_user_id
            ))
            .form(&[
                ("creation_id", creation_id),
                ("access_token", &self.user_access_token),
            ])
            .send()?
            .text()?;

        info!(?media_publish_response, "Instagram Media Published!");

        Ok(serde_json::from_str(&media_publish_response)?)
    }
}
