use super::session::Session;
use super::message::{Group, GroupMember, FriendMember};
use super::error::{Result, assert};
use serde::de::DeserializeOwned;

impl Session {
    fn get_list<D>(&self, name: &'static str) -> Result<Vec<D>> where
        D: DeserializeOwned {
        let resp = self.client.get(&self.url(&format!("/{}List?sessionKey={}", name, self.key)))
            .send()?
            .json()?;

        Ok(resp)
    }

    pub fn friend_list(&self) -> Result<Vec<FriendMember>> {
        self.get_list("friend")
    }

    pub fn group_list(&self) -> Result<Vec<Group>> {
        self.get_list("group")
    }

    pub fn group_member_list(&self) -> Result<Vec<GroupMember>> {
        self.get_list("member")
    }
}