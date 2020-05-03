//! You can use [`fetch_newest_message`] (or other similar function) to get a message from the server,
//! or use [`send_message`] to send a message to someone by the server.
//!
//! ## The Message Structure
//!
//! [`Message`] is a struct contains the message what you want to send, it has following fields:
//!
//! * quote: [`quote`] is an optional property, if you want to reply to someone, you can use [`quote`].
//! * message_chain: Message chain is the content of a [`Message`], it contains [`SingleMessage`]s, we will introduce it below
//!
//! ## SingleMessage
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
//!
//! ## MessagePackage
//!
//! [`MessagePackage`] is the message which you received from the server, it can be a message or an event, the variants are defined in the [`MessagePackage`]
//!
//! There are two primary message variant: [`GroupMessage`] and [`FriendMessage`].
//!
//! * GroupMessage: the message from a group, it contains a sender ([`GroupMember`]) and a group struct ([`Group`])
//! * FriendMessage: the message from a friend, it just contains a sender ([`FriendMember`])
//!
//! Other message variant or event variants information can found in [this](https://github.com/mamoe/mirai-api-http/blob/master/EventType.md).
//!
//! ## MessageBuilder
//!
//! [`MessageBuilder`] provides a way to build a message in builder-like flavor.
//!
//! like this:
//!
//! ```rust
//! use mirai::message::MessageBuilder;
//!
//! let message = MessageBuilder::new()
//!                 .append_message("Hello".into())
//!                 .build();
//! ```
//!
//! ## MessageChannel
//!
//! To send a message to others, you need to specify a channel which the message send to. There are three channel you can use:
//!
//! * Group: send a message to a group
//! * Friend: send a message to a friend
//! * Temp: send a message to a group member
//!

use serde::{Serialize, Deserialize};
use serde::export::fmt::Debug;

use crate::{Target, Code};
use crate::session::Session;
use crate::error::{Result, assert, ImpossibleError, ClientError};

pub type MessageChain = Vec<SingleMessage>;
pub type MessageId = i64;
pub type TimeStamp = u64;

#[derive(Deserialize, Debug, Clone)]
pub enum Permission {
    #[serde(rename = "ADMINISTRATOR")]
    Administrator,

    #[serde(rename = "OWNER")]
    Owner,

    #[serde(rename = "MEMBER")]
    Member,
}

#[serde(tag = "type")]
#[derive(Debug, Clone, Deserialize)]
pub enum MessagePackage {
    GroupMessage {
        #[serde(rename = "messageChain")]
        message_chain: MessageChain,
        sender: GroupMember,
    },

    FriendMessage {
        #[serde(rename = "messageChain")]
        message_chain: MessageChain,
        sender: FriendMember,
    },

    GroupRecallEvent {
        #[serde(rename = "authorId")]
        author_id: Target,
        #[serde(rename = "messageId")]
        message_id: MessageId,
        time: TimeStamp,
        group: Group,
        operator: GroupMember,
    },

    FriendRecallEvent {
        #[serde(rename = "authorId")]
        author_id: Target,
        #[serde(rename = "messageId")]
        message_id: MessageId,
        time: TimeStamp,
        operator: Target,
    },

    BotOnlineEvent { qq: Target },
    BotOfflineEventActive { qq: Target },
    BotOfflineEventForce { qq: Target },
    BotOfflineEventDropped { qq: Target },
    BotReloginEvent { qq: Target },

    BotGroupPermissionChangeEvent {
        origin: Permission,
        current: Permission,
        group: Group,
    },

    BotMuteEvent {
        #[serde(rename = "durationSeconds")]
        duration: u32,
        operator: GroupMember,
    },

    BotUnmuteEvent {
        operator: GroupMember
    },

    BotJoinGroupEvent { group: Group },
    BotLeaveEventActive { group: Group },
    BotLeaveEventKick { group: Group },

    GroupNameChangeEvent {
        origin: String,
        current: String,
        group: Group,
        operator: GroupMember,
    },

    GroupEntranceAnnouncementChangeEvent {
        origin: String,
        current: String,
        group: Group,
        operator: GroupMember,
    },

    #[serde(other)]
    Unsupported,
}

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
    Quote {
        id: MessageId,
        #[serde(rename = "groupId")]
        group_id: Target,
        #[serde(rename = "senderId")]
        sender_id: Target,
        #[serde(rename = "targetId")]
        target_id: Target,
        origin: MessageChain,
    },
    At {
        target: Target,
        display: String,
    },
    Image {
        #[serde(rename = "imageId")]
        image_id: Option<String>,
        url: Option<String>,
        path: Option<String>,
    },
    FlashImage {
        #[serde(rename = "imageId")]
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

#[derive(Debug, Clone, Deserialize)]
pub struct GroupMember {
    pub id: Target,
    #[serde(rename = "memberName")]
    pub member_name: String,
    pub permission: Permission,
    pub group: Group,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendMember {
    pub id: Target,
    #[serde(rename = "nickname")]
    pub nick_name: String,
    pub remark: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub id: Target,
    pub name: String,
    pub permission: Permission,
}

#[derive(Debug, Clone)]
pub enum MessageChannel {
    Friend(Target),
    Group(Target),
    Temp { qq: Target, group: Target },
}

impl MessageChannel {
    pub fn group(self) -> Result<Target> {
        if let MessageChannel::Group(group) = self {
            Ok(group)
        } else {
            Err(ClientError("Expect Group Target".to_string()))
        }
    }

    pub fn friend(self) -> Result<Target> {
        if let MessageChannel::Friend(friend) = self {
            Ok(friend)
        } else {
            Err(ClientError("Expect Friend Target".to_string()))
        }
    }

    /// returns (qq, group)
    pub fn temp(self) -> Result<(Target, Target)> {
        if let MessageChannel::Temp { qq, group } = self {
            Ok((qq, group))
        } else {
            Err(ClientError("Expect Temp Target".to_string()))
        }
    }
}

pub trait AsGroupChannel {
    fn as_group_channel(&self) -> MessageChannel;
}

pub trait AsFriendChannel {
    fn as_friend_channel(&self) -> MessageChannel;
}

pub trait AsTempChannel {
    fn as_temp_channel(&self) -> MessageChannel;
}

impl AsGroupChannel for Target {
    fn as_group_channel(&self) -> MessageChannel {
        MessageChannel::Group(self.clone())
    }
}

impl AsFriendChannel for Target {
    fn as_friend_channel(&self) -> MessageChannel {
        MessageChannel::Friend(self.clone())
    }
}

impl AsTempChannel for (Target, Target) {
    fn as_temp_channel(&self) -> MessageChannel {
        MessageChannel::Temp { qq: self.0, group: self.1 }
    }
}

impl AsFriendChannel for GroupMember {
    fn as_friend_channel(&self) -> MessageChannel {
        self.id.as_friend_channel()
    }
}

impl AsTempChannel for GroupMember {
    fn as_temp_channel(&self) -> MessageChannel {
        (self.id, self.group.id).as_temp_channel()
    }
}

impl AsGroupChannel for Group {
    fn as_group_channel(&self) -> MessageChannel {
        self.id.as_group_channel()
    }
}

impl AsFriendChannel for FriendMember {
    fn as_friend_channel(&self) -> MessageChannel {
        self.id.as_friend_channel()
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub quote: Option<MessageId>,
    pub message_chain: Vec<SingleMessage>,
}

impl Message {
    pub fn new(quote: Option<MessageId>, message_chain: &Vec<SingleMessage>) -> Message {
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

// -------------------------------------------------- Message Builder
#[derive(Debug, Clone)]
pub struct MessageBuilder {
    quote: Option<MessageId>,
    message_chain: Vec<SingleMessage>,
}

/// # MessageBuilder
///
/// `MessageBuilder` can build a `Message` by builder-like flavor.
///
/// When invoking [build] function, `MessageBuilder` need: [target] and a non-empty [message_chain],
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

    pub fn quote(mut self, quote: MessageId) -> MessageBuilder {
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

// -------------------------------------------------- send message
#[derive(Serialize)]
struct RichSendMsgRequest<'mc> {
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
        let mut req = RichSendMsgRequest {
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

// -------------------------------------------------- receive message
impl Session {
    async fn get_message(&self, is_fetch: bool, is_newest: bool, count: usize) -> Result<Vec<MessagePackage>> {
        #[derive(Deserialize)]
        struct Response {
            code: Code,
            data: Vec<MessagePackage>,
        }

        let url = format!("/{is_fetch}{is_newest}Message?sessionKey={sessionKey}&count={count}",
                          is_fetch = if is_fetch { "fetch" } else { "peek" },
                          is_newest = if is_newest { "Latest" } else { "" },
                          sessionKey = self.key,
                          count = count);

        let response: Response = self.client.get(&self.url(&url))
            .send().await?
            .json().await?;

        assert(response.code, if is_fetch { "Fetching" } else { "Peeking" })?;

        Ok(response.data)
    }

    pub async fn fetch_newest_message(&self, count: usize) -> Result<Vec<MessagePackage>> {
        self.get_message(true, true, count).await
    }

    pub async fn fetch_message(&self, count: usize) -> Result<Vec<MessagePackage>> {
        self.get_message(true, false, count).await
    }

    pub async fn peek_newest_message(&self, count: usize) -> Result<Vec<MessagePackage>> {
        self.get_message(false, true, count).await
    }

    pub async fn peek_message(&self, count: usize) -> Result<Vec<MessagePackage>> {
        self.get_message(false, false, count).await
    }
}