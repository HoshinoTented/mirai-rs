use serde::Deserialize;

use crate::Target;

#[derive(Deserialize, Debug, Clone)]
pub enum Permission {
    #[serde(rename = "ADMINISTRATOR")]
    Administrator,

    #[serde(rename = "OWNER")]
    Owner,

    #[serde(rename = "MEMBER")]
    Member,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupMember {
    pub(crate) id: Target,
    #[serde(rename = "memberName")]
    pub(crate) member_name: String,
    pub(crate) permission: Permission,
    pub(crate) group: Group,
}

impl PartialEq for GroupMember {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.group == other.group
    }
}

impl Eq for GroupMember {}

impl GroupMember {
    pub fn id(&self) -> Target {
        self.id
    }

    pub fn member_name(&self) -> String {
        self.member_name.clone()
    }

    pub fn permission(&self) -> Permission {
        self.permission.clone()
    }

    pub fn group(&self) -> Group {
        self.group.clone()
    }
}

#[serde(untagged)]
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub enum Sender {
    Friend(FriendMember),
    Group(GroupMember),
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendMember {
    pub(crate) id: Target,
    #[serde(rename = "nickname")]
    pub(crate) nick_name: String,
    pub(crate) remark: String,
}

impl PartialEq for FriendMember {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for FriendMember {}

impl FriendMember {
    pub fn id(&self) -> Target {
        self.id
    }

    pub fn nick_name(&self) -> String {
        self.nick_name.clone()
    }

    pub fn remark(&self) -> String {
        self.remark.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub(crate) id: Target,
    pub(crate) name: String,
    pub(crate) permission: Permission,
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Group {}

impl Group {
    pub fn id(&self) -> Target {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn permission(&self) -> Permission {
        self.permission.clone()
    }
}