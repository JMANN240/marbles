use std::{error::Error, thread, time::Duration};

use reqwest::blocking::{multipart, Client};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadBody {
    pub snippet: UploadBodySnippet,
    pub status: UploadBodyStatus,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadBodySnippet {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadBodyStatus {
    pub privacy_status: PrivacyStatus,
    pub self_declared_made_for_kids: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PrivacyStatus {
    Public,
    Private,
    Unlisted,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoResponse {
    id: String,
}

pub struct YouTubePoster {
    client: Client,
    token: String,
}

impl YouTubePoster {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub fn upload(&self, path: &str, title: String, description: String, tags: Vec<String>) -> Result<VideoResponse, Box<dyn Error>> {
        info!("Posting to YouTube...");

        let form = multipart::Form::new()
            .part(
                "application/json",
                multipart::Part::text(
                    serde_json::to_string(
                        &UploadBody {
                            snippet: UploadBodySnippet { title, description, tags, category_id: "24".to_string() },
                            status: UploadBodyStatus { privacy_status: PrivacyStatus::Public, self_declared_made_for_kids: false }
                        }
                    ).unwrap()
                )
            )
            .part("video/mp4", multipart::Part::file(path).unwrap());

        info!(?form, "Request Body");

        let video_response = self.client
            .post("https://www.googleapis.com/youtube/v3/videos")
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&[
                ("part", "snippet,status"),
            ])
            .multipart(form)
            .send()?
            .text()?;

        info!(?video_response, "Posted to YouTube!");

        Ok(serde_json::from_str(&video_response)?)
    }
}
