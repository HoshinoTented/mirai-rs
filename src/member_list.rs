use crate::session::Session;
use crate::message::{Group, GroupMember, FriendMember};
use crate::error::{Result};
use serde::de::DeserializeOwned;

impl Session {
    async fn get_list<D>(&self, name: &'static str) -> Result<Vec<D>> where
        D: DeserializeOwned {
        let resp = self.client.get(&self.url(&format!("/{}List?sessionKey={}", name, self.key)))
            .send()
            .await?
            .json()
            .await?;

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