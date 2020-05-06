//! Single Message is the element of MessageChain, when you want to send a message, you need to construct them.
//!
//! # SingleMessage
//!
//! [`SingleMessage`] is the element of [`MessageChain`], it has many variants:
//!
//! * Source: It contains a message-id and timestamp, but in common you don't need to use it, it only returns from the server.
//! * Plain: It contains plain text, [`Plain`] message is common, and most frequently uses.
//! * Quote: It is similar to [`Source`] variant, only returns from the server. It means this message quoted another message.
//! * At: You can use [`At`] variant when you want this message notice somebody, the [`display`] property is how this [`At`] message displays.
//! * Image | FlashImage: [`Image`] and [`FlashImage`] are similar, they both send an image message, but [`FlashImage`] has a time limitation.
//!                       Both of them have three property: [`image_id`], [`url`] and [`path`],
//!                       [`image_id`] is the id of an image which saved in Tencent server,
//!                       [`url`] is a url that points to an image,
//!                       [`path`] is a path that points to an image in the server.
//!                       They also have priority, [`image_id`] > [`url`] > [`path`].
//! * Xml | Json | App | Poke: These message are not very commonly used, you can see [this](https://github.com/mamoe/mirai-api-http/blob/master/MessageType.md) for more information.

use serde::{Serialize, Deserialize};

use crate::message::{MessageId, TimeStamp, MessageChain};
use crate::Target;

#[serde(tag = "type")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SingleMessage {
    Source {
        id: MessageId,
        time: TimeStamp,
    },
    Plain {
        text: String
    },
    #[serde(rename_all = "camelCase")]
    Quote {
        id: MessageId,
        group_id: Target,
        sender_id: Target,
        target_id: Target,
        origin: MessageChain,
    },
    At {
        target: Target,
        display: String,
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

impl From<String> for SingleMessage {
    fn from(str: String) -> Self {
        SingleMessage::Plain {
            text: str
        }
    }
}

impl From<&str> for SingleMessage {
    fn from(str: &str) -> Self {
        SingleMessage::from(str.to_string())
    }
}

impl ToString for SingleMessage {
    fn to_string(&self) -> String {
        match self {
            SingleMessage::Source { id, time: _ } => format!("[mirai:source:{}]", id),
            SingleMessage::Plain { text } => text.clone(),
            SingleMessage::Quote { id, group_id: _, sender_id: _, target_id: _, origin: _ } => format!("[mirai:quote:{}]", id),
            SingleMessage::At { target, display: _ } => format!("[mirai:at:{}]", target),
            SingleMessage::Image { .. } => "[Image]".to_string(),
            SingleMessage::FlashImage { .. } => "[FlashImage]".to_string(),
            SingleMessage::Xml { xml } => format!("[mirai:xml:{}]", xml),
            SingleMessage::Json { json } => format!("[mirai:json:{}]", json),
            SingleMessage::App { content } => format!("[mirai:app:{}]", content),
            SingleMessage::Poke { name } => format!("[mirai:poke:{}]", name),
            SingleMessage::Unsupported => format!("{:?}", SingleMessage::Unsupported),
        }
    }
}