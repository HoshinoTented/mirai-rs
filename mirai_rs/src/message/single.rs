//! Single Message is the element of MessageChain, when you want to send a message, you need to construct them.
//!
//! # SingleMessage
//!
//! [`SingleMessage`] is the element of [`MessageChain`], it has many variants:
//!
//! * Plain: It contains plain text, [`Plain`] message is common, and most frequently uses.
//! * At: You can use [`At`] variant when you want this message notice somebody, the [`display`] property is how this [`At`] message displays.
//! * AtAll: This message can only be received from a group.
//! * Face: A face (aka expression) message element, if you want to construct it, you need provide at least one of [face_id] or [name].
//! * Image | FlashImage: [`Image`] and [`FlashImage`] are similar, they both send an image message, but [`FlashImage`] has a time limitation.
//!                       Both of them have three property: [`image_id`], [`url`] and [`path`],
//!                       [`image_id`] is the id of an image which saved in Tencent server,
//!                       [`url`] is a url that points to an image,
//!                       [`path`] is a path that points to an image in the server.
//!                       They also have priority, [`image_id`] > [`url`] > [`path`].
//! * Xml | Json | App | Poke: These message are not very commonly used, you can see [this](https://github.com/mamoe/mirai-api-http/blob/master/MessageType.md) for more information.

use serde::{Serialize, Deserialize};

use crate::Target;
use serde::export::fmt::Display;
use serde::export::Formatter;

#[serde(tag = "type")]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum SingleMessage {
    Plain {
        text: String
    },
    At {
        target: Target,
        display: String,
    },
    AtAll,
    #[serde(rename_all = "camelCase")]
    Face {
        face_id: Option<i32>,
        name: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Image {
        image_id: Option<String>,
        url: Option<String>,
        path: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    FlashImage {
        image_id: Option<String>,
        url: Option<String>,
        path: Option<String>,
    },
    Xml {
        xml: String
    },
    Json {
        json: String
    },
    App {
        content: String
    },
    Poke {
        name: String
    },

    #[serde(other)]
    Unsupported,
}

impl<S: AsRef<str>> From<S> for SingleMessage {
    fn from(str: S) -> Self {
        SingleMessage::Plain {
            text: str.as_ref().to_string()
        }
    }
}

impl Display for SingleMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SingleMessage::Plain { text } => text.clone(),
            SingleMessage::At { target, display } => format!("[at:{}@{}]", target, display),
            SingleMessage::Image { .. } => "[image]".to_string(),
            SingleMessage::FlashImage { .. } => "[flash_image]".to_string(),
            SingleMessage::Xml { xml } => format!("[xml:{}]", xml),
            SingleMessage::Json { json } => format!("[json:{}]", json),
            SingleMessage::App { content } => format!("[app:{}]", content),
            SingleMessage::Poke { name } => format!("[poke:{}]", name),
            SingleMessage::Unsupported => format!("{:?}", SingleMessage::Unsupported),
            SingleMessage::AtAll => "[atall]".to_string(),
            SingleMessage::Face { face_id, name } => {
                let s = if let Some(id) = face_id {
                    id.to_string()
                } else if let Some(name) = name {
                    name.clone()
                } else {
                    panic!("id == None && name == None");
                };

                format!("[ce:{}]", s)
            }
        };

        f.write_str(&s)
    }
}