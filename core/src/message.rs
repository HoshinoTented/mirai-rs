use serde::{Serialize, Deserialize};
use serde::export::fmt::Debug;

use crate::{Target, Code};
use crate::session::Session;
use crate::error::{Result, assert, ImpossibleError};

pub type MessageChain = Vec<SingleMessage>;
pub type MessageId = i64;
pub type TimeStamp = u64;

/// # Permission
///
/// Permission enum structure, this can be found in `GroupMember` and `Group`.
#[derive(Deserialize, Debug, Clone)]
pub enum Permission {
    #[serde(rename = "ADMINISTRATOR")]
    Administrator,

    #[serde(rename = "OWNER")]
    Owner,

    #[serde(rename = "MEMBER")]
    Member,
}

/// # MessagePackage
///
/// `MessagePackage` will be returned from `Session::get_message`.
/// It contains messages (or events) which the bot received.
///
/// ## Variants
///
/// ### GroupMessage
///
/// it contains a message chain (`message_chain`) and a sender (`GroupMember`)
///
/// ### FriendMessage
///
/// it contains a message chain (`message_chain`) and a sender (`FriendMember`)
///
/// ### GroupRecallEvent
///
/// it means `operator` recall a group message (`message_id`) which `author_id` sent
///
/// ### FriendRecallEvent
///
/// the same as above
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
    Unsupported
}

/// # SingleMessage
///
/// The element of `MessageChain`.
///
/// ## Variants
///
/// ### Source
///
/// `Source` variant contains message id and timestamp.
///
/// ### Plain
///
/// `Plain` text message, no special.
///
/// ### Quote
///
/// * `id`: quoted message id
/// * `group_id`: what group this message send to
/// * `sender_id`: sender id
/// * `target_id`: the sender of quoted message
/// * `origin`: the message chain of quoted message
///
/// ### At
///
/// * `target`: target member id
/// * `display`
///
/// ### (Flash)?Image
///
/// * `image_id`: image id
/// * `url`: url which points an image
/// * `path`: path which points an image
///
/// ### Xml
///
/// xml message
///
/// ### Json
///
/// json message
///
/// ### App
///
/// app message
///
/// ### Poke
///
/// poke message
///
/// ### Unsupported
///
/// the message which mirai-rs not supports
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
    Unsupported
}

/// # GroupMember
///
/// This struct can get from `MessagePackage::Group`
///
/// ## Attributes
///
/// * `id`: the id of sender
/// * `member_name`: sender's name
/// * `permission`: sender's permission in this group
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

/// # Group
///
/// This struct can get from `GroupMember`
///
/// ## Attribute
///
/// * `id`: the group id
/// * `name`: the group name
/// * `permission`: bot's permission in this group
#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub id: Target,
    pub name: String,
    pub permission: Permission,
}

/// # Message
#[derive(Debug, Serialize)]
pub struct Message {
    pub(crate) target: Target,
    #[serde(rename = "messageChain")]
    pub(crate) message_chain: Vec<SingleMessage>,
}

impl Message {
    pub fn new(target: Target, message_chain: &Vec<SingleMessage>) -> Message {
        Message {
            target,
            message_chain: message_chain.to_vec(),
        }
    }
}

/// send message
impl Session {
    async fn send_message<'mc>(&self, message_type: &str, message: &'mc Message) -> Result<u64> {
        #[derive(Serialize)]
        struct SendMessageRequest<'mc> {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
            #[serde(rename = "messageChain")]
            message_chain: &'mc MessageChain,
        }

        #[derive(Deserialize)]
        struct SendMessageResponse {
            code: Code,
            //            msg: String,
            #[serde(rename = "messageId")]
            message_id: Option<u64>,
        }

        let req = SendMessageRequest {
            session_key: self.key.clone(),
            target: message.target,
            message_chain: &message.message_chain,
        };

        let resp: SendMessageResponse = self.client.post(&self.url(&format!("/send{}Message", message_type)))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Sending")?;

        resp.message_id.ok_or(ImpossibleError("messageId is None".to_string()))
    }

    pub async fn send_group_message(&self, message: &Message) -> Result<u64> {
        self.send_message("Group", message).await
    }

    pub async fn send_friend_message(&self, message: &Message) -> Result<u64> {
        self.send_message("Friend", message).await
    }

    pub async fn send_temp_message(&self, message: &Message) -> Result<u64> {
        self.send_message("Temp", message).await
    }
}

/// receive message
impl Session {
    async fn get_message(&self, is_fetch: bool, is_newest: bool, count: usize) -> Result<Vec<MessagePackage>> {
        #[derive(Deserialize)]
        struct GetMessageResponse {
            code: Code,
            data: Vec<MessagePackage>,
        }

        let url = format!("/{is_fetch}{is_newest}Message?sessionKey={sessionKey}&count={count}",
                          is_fetch = if is_fetch { "fetch" } else { "peek" },
                          is_newest = if is_newest { "Latest" } else { "" },
                          sessionKey = self.key,
                          count = count);

        let response: GetMessageResponse = self.client.get(&self.url(&url))
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