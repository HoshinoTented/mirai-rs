//! You can use some function in this mod to send message with a session

use serde::{Serialize, Deserialize};

use crate::{Target, Code};
use crate::message::{MessageId, MessageChain, Message};
use crate::session::Session;
use crate::message::channel::MessageChannel;
use crate::error::{Result, assert, ImpossibleError};

#[derive(Serialize)]
struct SendMsgRequest<'mc> {
    #[serde(rename = "sessionKey")]
    session_key: String,
    qq: Option<Target>,
    group: Option<Target>,
    quote: Option<MessageId>,
    #[serde(rename = "messageChain")]
    message_chain: &'mc MessageChain,
}

#[derive(Deserialize)]
struct SendMsgResponse {
    code: Code,
    //            msg: String,
    #[serde(rename = "messageId")]
    message_id: Option<u64>,
}

impl Session {
    pub async fn send_message(&self, channel: &MessageChannel, message: &Message) -> Result<u64> {
        let mut req = SendMsgRequest {
            session_key: self.key.clone(),
            qq: None,
            group: None,
            quote: message.quote,
            message_chain: &message.message_chain,
        };

        let message_type = match channel {
            MessageChannel::Group(group) => {
                req.group = Some(*group);

                "Group"
            }

            MessageChannel::Friend(friend) => {
                req.qq = Some(*friend);

                "Friend"
            }

            MessageChannel::Temp { qq, group } => {
                req.qq = Some(*qq);
                req.group = Some(*group);

                "Temp"
            }
        };

        let resp: SendMsgResponse = self.client.post(&self.url(&format!("/send{}Message", message_type)))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Sending")?;

        resp.message_id.ok_or(ImpossibleError("messageId is None".to_string()))
    }
}