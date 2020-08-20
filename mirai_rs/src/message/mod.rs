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
pub mod message;

pub use channel::MessageChannel;
pub use single::SingleMessage;
pub use event::EventPacket;
pub use element::{Group, GroupMember, FriendMember};
pub use single::SingleMessage::*;
pub use message::{TimeStamp, MessageID, Message, MessageChain};