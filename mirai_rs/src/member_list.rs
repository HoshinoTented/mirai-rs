//! This mod only provides a way to get a list of member (or group)
//!

use crate::session::Session;
use crate::error::HttpResult;
use serde::de::DeserializeOwned;
use crate::message::element::{FriendMember, Group, GroupMember};
use crate::Target;

impl Session {
    async fn get_list<D>(&self, path: String) -> HttpResult<Vec<D>> where
        D: DeserializeOwned {
        let resp = self.client()
            .get(&self.url(&path))
            .send().await?
            .json().await?;

        Ok(resp)
    }

    /// Get the friend list of the bound QQ
    pub async fn friend_list(&self) -> HttpResult<Vec<FriendMember>> {
        self.get_list(format!("/friendList?sessionKey={}", self.key)).await
    }

    /// Get the group list of the bound QQ
    pub async fn group_list(&self) -> HttpResult<Vec<Group>> {
        self.get_list(format!("/groupList?sessionKey={}", self.key)).await
    }

    /// Get
    pub async fn group_member_list(&self, target: Target) -> HttpResult<Vec<GroupMember>> {
        self.get_list(format!("/memberList?sessionKey={}&target={}", self.key, target)).await
    }
}