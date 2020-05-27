//! This mod contains two functions impl for session, one to get the mirai-api-http setting, the other for modifying it.
//!
//! # Config
//!
//! A [`Config`] contains two member variables.
//!
//! * cash_size: the cashsize of mirai-api-http server, too small cache will lead to failure of reference reply and recall messages
//! * enable_websocket: whether websocket is open
//!

use serde::{Deserialize, Serialize};

use crate::error::{assert, Result};
use crate::session::{CommonResponse, Session};
use crate::CacheSize;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_size: CacheSize,
    pub enable_websocket: bool,
}

impl Session {
    /// Return config of mirai-api-http server.
    pub async fn get_config(&self) -> Result<Config> {
        let config: Config = self
            .client()
            .get(&self.url(&format!("/config?sessionKey={}", self.key)))
            .send()
            .await?
            .json()
            .await?;

        Ok(config)
    }
    /// Return the result of modify mirai-api-http server.
    pub async fn modify_config(&self, cache_size: CacheSize, enable_websocket: bool) -> Result<()> {
        #[serde(rename_all = "camelCase")]
        #[derive(Serialize)]
        struct Request {
            session_key: String,
            cache_size: CacheSize,
            enable_websocket: bool,
        }

        let req = Request {
            session_key: self.key.clone(),
            cache_size: cache_size,
            enable_websocket: enable_websocket,
        };

        let resp: CommonResponse = self
            .client()
            .post(&self.url("/config"))
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        assert(resp.code, "ModifyConfig")
    }
}
