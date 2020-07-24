//! Message channel is a target which you want to send to.
//!
//! # MessageChannel
//!
//! To send a message to others, you need to specify a channel which the message send to. There are three channel you can use:
//!
//! * Group: send a message to a group
//! * Friend: send a message to a friend
//! * Temp: send a message to a group member
//!
//! Then you can:
//!
//! ```ignore
//! use mirai::message::{channel::MessageChannel, MessageBuilder};
//! use mirai::session::Session;
//!
//! let session: Session = my_session;
//! let target_channel = MessageChannel::Group(group);
//! let message = MessageBuilder::new().append_message("Hello".into()).build().unwrap();
//!
//! session.send_message(target_channel, &message).await.unwrap();
//! ```

use crate::Target;
use crate::error::{Result, client_error};
use crate::message::element::{GroupMember, Group, FriendMember};

/// Mirai-api-http provides three channel to send message, [Friend], [Group] and [Temp].
#[derive(Debug, Clone, Copy)]
pub enum MessageChannel {
    Friend(Target),
    Group(Target),
    Temp { qq: Target, group: Target },
}

impl MessageChannel {
    /// Return `Ok(group)` if this channel is [Group]
    pub fn group(self) -> Result<Target> {
        if let MessageChannel::Group(group) = self {
            Ok(group)
        } else {
            Err(client_error("Expect Group Target"))
        }
    }

    /// Return `Ok(QQ)` if this channel is [Friend]
    pub fn friend(self) -> Result<Target> {
        if let MessageChannel::Friend(friend) = self {
            Ok(friend)
        } else {
            Err(client_error("Expect Friend Target"))
        }
    }

    /// Return `Ok((QQ, Group))` if this channel is [Temp]
    pub fn temp(self) -> Result<(Target, Target)> {
        if let MessageChannel::Temp { qq, group } = self {
            Ok((qq, group))
        } else {
            Err(client_error("Expect Temp Target"))
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