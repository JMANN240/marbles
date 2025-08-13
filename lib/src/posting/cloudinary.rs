use std::{collections::HashMap, env, error::Error};

use itertools::Itertools;
use reqwest::blocking::{Client, multipart};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct CloudinaryUploadResponse {
    pub public_id: String,
    pub secure_url: String,
    pub duration: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudinaryDestroyResponse {
    pub result: String,
}

pub struct Cloudinary {
    client: Client,
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

impl Cloudinary {
    pub fn new(cloud_name: String, api_key: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            cloud_name,
            api_key,
            api_secret,
        }
    }

    pub fn from_env() -> Self {
        Cloudinary::new(
            env::var("CLOUDINARY_CLOUD_NAME")
                .expect("Missing CLOUDINARY_CLOUD_NAME environment variable!"),
            env::var("CLOUDINARY_API_KEY")
                .expect("Missing CLOUDINARY_API_KEY environment variable!"),
            env::var("CLOUDINARY_API_SECRET")
                .expect("Missing CLOUDINARY_API_SECRET environment variable!"),
        )
    }

    pub fn generate_signature(&self, params: HashMap<&str, String>) -> String {
        let mut sorted_keys: Vec<&str> = params.keys().cloned().collect();

        sorted_keys.sort();

        let sorted_params = sorted_keys
            .iter()
            .map(|key| format!("{}={}", key, &params[key]))
            .join("&");

        let string_to_sign = format!("{}{}", sorted_params, self.api_secret);

        let mut hasher = Sha1::new();
        hasher.update(string_to_sign.as_bytes());

        hex::encode(hasher.finalize())
    }

    pub fn post(&self, path: &str) -> Result<CloudinaryUploadResponse, Box<dyn Error>> {
        let timestamp = chrono::Utc::now().timestamp();

        let mut params = HashMap::new();
        params.insert("timestamp", timestamp.to_string());

        let signature = self.generate_signature(params);

        let buffer = std::fs::read(path).unwrap();
        let part = multipart::Part::bytes(buffer).file_name("file");

        let form = multipart::Form::new()
            .text("timestamp", timestamp.to_string())
            .text("signature", signature)
            .text("api_key", self.api_key.clone())
            .part("file", part);

        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/video/upload",
            &self.cloud_name
        );

        info!("Uploading to Cloudinary...");

        let cloudinary_response = self.client.post(url).multipart(form).send()?.text()?;

        info!(?cloudinary_response, "Uploaded to Cloudinary!");

        Ok(serde_json::from_str(&cloudinary_response)?)
    }

    pub fn delete(&self, public_id: &str) -> Result<CloudinaryDestroyResponse, Box<dyn Error>> {
        let timestamp = chrono::Utc::now().timestamp();

        let mut params = HashMap::new();
        params.insert("public_id", public_id.to_string());
        params.insert("timestamp", timestamp.to_string());

        let signature = self.generate_signature(params);

        let mut params = HashMap::new();
        params.insert("public_id", public_id.to_string());
        params.insert("timestamp", timestamp.to_string());
        params.insert("signature", signature);
        params.insert("api_key", self.api_key.clone());

        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/video/destroy",
            &self.cloud_name
        );

        info!("Destroying Cloudinary Asset...");

        let cloudinary_destroy_response = self.client.post(url).query(&params).send()?.text()?;

        info!(?cloudinary_destroy_response, "Cloudinary Asset Destroyed!");

        Ok(serde_json::from_str(&cloudinary_destroy_response)?)
    }
}
