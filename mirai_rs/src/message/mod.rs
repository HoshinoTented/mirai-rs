//! You can use [`fetch_newest_message`] (or other similar function in [receive] mod) to get a message from the server,
//! or use [`send_message`] (which in [send] mod) to send a message to someone by the server.
//!
//! ## The Message Structure
//!
//! [`Message`] is a struct contains the message what you want to send, it has following fields:
//!
//! * quote: [`quote`] is an optional property, if you want to reply to someone, you can use [`quote`].
//! * message_chain: Message chain is the content of a [`Message`], it contains [`SingleMessage`]s.
//!
//! ## MessageBuilder
//!
//! [`MessageBuilder`] provides a way to build a message in builder-like flavor.
//!
//! like this:
//!
//! ```ignore
//! use mirai::message::MessageBuilder;
//!
//! let message = MessageBuilder::new()
//!                 .append_message("Hello".into())
//!                 .build();
//! ```
//!

pub mod event;
pub mod single;
pub mod element;
pub mod channel;
pub mod send;
pub mod receive;
pub mod parse;

use crate::error::Result;

pub use channel::MessageChannel;
pub use single::SingleMessage;
pub use event::EventPacket;
pub use element::{Group, GroupMember, FriendMember};

pub type MessageChain = Vec<SingleMessage>;
pub type MessageID = i64;
pub type TimeStamp = u64;

#[derive(Debug, Clone)]
pub struct Message {
    pub quote: Option<MessageID>,
    pub message_chain: MessageChain,
}

impl Message {
    pub fn new(quote: Option<MessageID>, message_chain: &MessageChain) -> Message {
        Message {
            quote,
            message_chain: message_chain.to_vec(),
        }
    }
}

impl From<MessageBuilder> for Message {
    fn from(builder: MessageBuilder) -> Self {
        builder.build().unwrap()
    }
}

impl From<SingleMessage> for Message {
    fn from(single: SingleMessage) -> Self {
        Message::new(None, &vec![single])
    }
}

#[derive(Debug, Clone)]
pub struct MessageBuilder {
    quote: Option<MessageID>,
    message_chain: MessageChain,
}

/// # MessageBuilder
///
/// `MessageBuilder` can build a `Message` by builder-like flavor.
///
/// When invoking [build] function, `MessageBuilder` needs a non-empty [message_chain],
/// if not, [build] function will returns an Error.
///
impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            quote: None,
            message_chain: Vec::new(),
        }
    }

    pub fn append_message(mut self, msg: SingleMessage) -> MessageBuilder {
        self.message_chain.push(msg);
        self
    }

    pub fn quote(mut self, quote: MessageID) -> MessageBuilder {
        self.quote = Some(quote);
        self
    }

    pub fn build(self) -> Result<Message> {
        Ok(Message {
            quote: self.quote,
            message_chain: self.message_chain,
        })
    }
}