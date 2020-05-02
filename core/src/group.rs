use serde::{Serialize, Deserialize};

use crate::session::{Session, CommonResponse};
use crate::error::{Result, assert};
use crate::Target;
use crate::message::MessageId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupConfig {
    pub name: String,
    pub announcement: String,
    #[serde(rename = "confessTalk")]
    pub confess_talk: bool,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "autoApprove")]
    pub auto_approve: bool,
    #[serde(rename = "anonymousChat")]
    pub anonymous_chat: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemberInfo {
    name: String,
    #[serde(rename = "specialTitle")]
    special_title: String,
}

impl Session {
    async fn do_mute_all(&self, target: Target, mute: bool) -> Result<()> {
        let path = if mute { "muteAll" } else { "unmuteAll" };

        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
        }

        let req = Request {
            session_key: self.key.clone(),
            target,
        };

        let resp: CommonResponse = self.client.post(&(self.url("/") + path))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, if mute { "MuteAll" } else { "UnmuteAll" })
    }

    pub async fn mute_all(&self, target: Target) -> Result<()> {
        self.do_mute_all(target, true).await
    }

    pub async fn unmute_all(&self, target: Target) -> Result<()> {
        self.do_mute_all(target, false).await
    }

    pub async fn mute(&self, group_id: Target, target: Target, seconds: u32) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
            #[serde(rename = "memberId")]
            member_id: Target,
            time: u32,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: group_id,
            member_id: target,
            time: seconds,
        };

        let resp: CommonResponse = self.client.post(&self.url("/mute"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Mute")
    }

    pub async fn unmute(&self, group_id: Target, target: Target) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
            #[serde(rename = "memberId")]
            member_id: Target,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: group_id,
            member_id: target,
        };

        let resp: CommonResponse = self.client.post(&self.url("/unmute"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Unmute")
    }

    pub async fn kick(&self, group_id: Target, target: Target, msg: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
            #[serde(rename = "memberId")]
            member_id: Target,
            msg: String,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: group_id,
            member_id: target,
            msg: msg.to_string(),
        };

        let resp: CommonResponse = self.client.post(&self.url("/kick"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Kick")
    }

    pub async fn quit(&self, group_id: Target) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: group_id,
        };

        let resp: CommonResponse = self.client.post(&self.url("/quit"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Quit")
    }

    pub async fn modify_group_config(&self, group_id: Target, config: &GroupConfig) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
            config: GroupConfig,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: group_id,
            config: config.clone(),
        };

        let resp: CommonResponse = self.client.post(&self.url("/groupConfig"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "ModifyGroupConfig")
    }

    pub async fn get_group_config(&self, group_id: Target) -> Result<GroupConfig> {
        let config: GroupConfig = self.client.get(&self.url(&format!("/groupConfig?sessionKey={}&target={}", self.key, group_id)))
            .send().await?
            .json().await?;

        Ok(config)
    }

    pub async fn modify_member_info(&self, group_id: Target, target: Target, info: &MemberInfo) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: Target,
            #[serde(rename = "memberId")]
            member_id: Target,
            info: MemberInfo,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: group_id,
            member_id: target,
            info: info.clone(),
        };

        let resp: CommonResponse = self.client.post(&self.url("/groupConfig"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "ModifyGroupConfig")
    }

    pub async fn get_member_info(&self, group_id: Target, target: Target) -> Result<MemberInfo> {
        let info: MemberInfo = self.client.get(&self.url(&format!("/memberInfo?sessionKey={}&target={}&memberId={}", self.key, group_id, target)))
            .send().await?
            .json().await?;

        Ok(info)
    }
}