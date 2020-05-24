//! You can use [fetch_newest_message] which in [receive] mod to receive EventPacket.
//!
//! # EventPacket
//!
//! [`EventPacket`] is the event which you received from the server, but it just a wrapper.
//!
//! The most commonly used event is: [`MessageEvent`]. [`MessageEvent`] has two primary variant:
//!
//! * GroupMessage: the message from a group, it contains a sender ([`GroupMember`]) and a group struct ([`Group`])
//! * FriendMessage: the message from a friend, it just contains a sender ([`FriendMember`])
//!
//! Other event variant information can found in [this](https://github.com/mamoe/mirai-api-http/blob/master/EventType.md).

use serde::Deserialize;
use serde_json::Value;

use crate::Target;
use crate::message::{MessageChain, MessageID, TimeStamp};
use crate::message::element::{GroupMember, FriendMember, Group};

#[serde(tag = "type")]
#[derive(Debug, Clone, Deserialize)]
pub enum MessageEvent {
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
}

impl MessageEvent {
    pub fn is_group(&self) -> bool {
        if let MessageEvent::GroupMessage { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_friend(&self) -> bool {
        if let MessageEvent::FriendMessage { .. } = self {
            true
        } else {
            false
        }
    }
}

#[serde(tag = "type")]
#[derive(Clone, Debug, Deserialize)]
pub enum RecallEvent {
    GroupRecallEvent {
        #[serde(rename = "authorId")]
        author_id: Target,
        #[serde(rename = "messageId")]
        message_id: MessageID,
        time: TimeStamp,
        group: Group,
        operator: Option<GroupMember>,          // Bot is operator if this field is None, the same as below
    },
    FriendRecallEvent {
        #[serde(rename = "authorId")]
        author_id: Target,
        #[serde(rename = "messageId")]
        message_id: MessageID,
        time: TimeStamp,
        operator: Target,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub enum BotLoginEventKind {
    BotOnlineEvent,
    BotOfflineEventActive,
    BotOfflineEventForce,
    BotOfflineEventDropped,
    BotReloginEvent,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotLoginEvent {
    #[serde(rename = "type")]
    kind: BotLoginEventKind,
    qq: Target,
}

#[derive(Clone, Debug, Deserialize)]
pub enum BotGroupEventKind {
    BotJoinGroupEvent,
    BotLeaveEventActive,
    BotLeaveEventKick,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotGroupEvent {
    #[serde(rename = "type")]
    kind: BotGroupEventKind,
    group: Group,
}

#[serde(tag = "type")]
#[derive(Clone, Debug, Deserialize)]
pub enum BotMuteEvent {
    BotMuteEvent {
        #[serde(rename = "durationSeconds")]
        duration: u32,
        operator: GroupMember,
    },
    BotUnmuteEvent {
        operator: GroupMember
    },
}

#[derive(Debug, Clone, Deserialize)]
pub enum GroupChangeEventKind {
    GroupNameChangeEvent,
    GroupEntranceAnnouncementChangeEvent,
    GroupMuteAllEvent,
    GroupAllowAnonymousChatEvent,
    GroupAllowMemberInviteEvent,
}

#[serde(untagged)]
#[derive(Clone, Debug, Deserialize)]
pub enum ChangeType {
    String {
        origin: String,
        current: String,
    },
    Bool {
        origin: bool,
        current: bool,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupChangeEvent {
    #[serde(rename = "type")]
    kind: GroupChangeEventKind,
    #[serde(flatten)]
    change: ChangeType,
    group: Group,
    operator: Option<GroupMember>,
}

#[serde(untagged)]
#[derive(Debug, Clone, Deserialize)]
pub enum EventPacket {
    MessageEvent(MessageEvent),
    BotLoginEvent(BotLoginEvent),
    BotMuteEvent(BotMuteEvent),
    RecallEvent(RecallEvent),
    GroupChangeEvent(GroupChangeEvent),

    Unsupported(Value),
}

impl EventPacket {
    pub fn is_message(&self) -> bool {
        match self {
            EventPacket::MessageEvent(_) => true,
            _ => false
        }
    }
}