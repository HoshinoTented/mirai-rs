use crate::session::Session;
use crate::error::HttpResult;
use crate::message::SingleMessage;

use reqwest::multipart::{Form, Part};
use reqwest::Body;

use serde::{Serialize, Deserialize};
use bytes::Bytes;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub image_id: String,
    pub url: String,
    pub path: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Friend,
    Group,
    Temp,
}

impl ToString for ImageType {
    fn to_string(&self) -> String {
        match self {
            ImageType::Friend => "friend",
            ImageType::Group => "group",
            ImageType::Temp => "temp",
        }.to_string()
    }
}

impl From<Image> for SingleMessage {
    fn from(img: Image) -> Self {
        SingleMessage::Image {
            image_id: Some(img.image_id),
            url: Some(img.url),
            path: Some(img.path),
        }
    }
}

impl Session {
    pub async fn upload_image(&self, image_type: ImageType, bytes: Bytes, file_name: String) -> HttpResult<Image> {
        let form = Form::new()
            .text("sessionKey", self.key.clone())
            .text("type", image_type.to_string())
            .part("img", Part::stream(Body::from(bytes)).file_name(file_name));

        let img: Image = self.client().post(&self.url("/uploadImage"))
            .multipart(form).send().await?
            .json().await?;

        Ok(img)
    }
}