//! You can use some function in this mod to send message with a session

use serde::{Serialize, Deserialize};

use crate::{Target, Code};
use crate::message::{MessageID, MessageChain, Message};
use crate::session::Session;
use crate::message::channel::MessageChannel;
use crate::error::{HttpResult, assert};


impl Session {
    /// Send a [message] to the given [channel], and returns a [MessageID]
    pub async fn send_message(&self, channel: MessageChannel, message: &Message) -> HttpResult<MessageID> {
        #[serde(rename_all = "camelCase")]
        #[derive(Serialize)]
        struct Request<'mc> {
            session_key: String,
            qq: Option<Target>,
            group: Option<Target>,
            quote: Option<MessageID>,
            message_chain: &'mc MessageChain,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Deserialize)]
        struct Response {
            code: Code,
            message_id: Option<MessageID>,
        }

        let mut req = Request {
            session_key: self.key.clone(),
            qq: None,
            group: None,
            quote: message.quote,
            message_chain: &message.message_chain,
        };

        let message_type = match channel {
            MessageChannel::Group(group) => {
                req.group = Some(group);

                "Group"
            }

            MessageChannel::Friend(friend) => {
                req.qq = Some(friend);

                "Friend"
            }

            MessageChannel::Temp { qq, group } => {
                req.qq = Some(qq);
                req.group = Some(group);

                "Temp"
            }
        };

        let resp: Response = self.client().post(&self.url(&format!("/send{}Message", message_type)))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Sending")?;

        Ok(resp.message_id.unwrap())
    }
}