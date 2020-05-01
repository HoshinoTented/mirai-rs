use serde::Serialize;

use crate::session::{Session, CommonResponse};
use crate::error::{Result, assert};
use crate::Target;

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
}