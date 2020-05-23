//! This mod only provides a way to get a list of member (or group)
//!

use crate::session::Session;
use crate::error::Result;
use serde::de::DeserializeOwned;
use crate::message::element::{FriendMember, Group, GroupMember};

impl Session {
    async fn get_list<D>(&self, name: &'static str) -> Result<Vec<D>> where
        D: DeserializeOwned {
        let resp = self.client().get(&self.url(&format!("/{}List?sessionKey={}", name, self.key)))
            .send().await?
            .json().await?;

        Ok(resp)
    }

    pub async fn friend_list(&self) -> Result<Vec<FriendMember>> {
        self.get_list("friend").await
    }

    pub async fn group_list(&self) -> Result<Vec<Group>> {
        self.get_list("group").await
    }

    pub async fn group_member_list(&self) -> Result<Vec<GroupMember>> {
        self.get_list("member").await
    }
}