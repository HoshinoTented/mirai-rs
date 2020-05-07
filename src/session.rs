//! This mod provides a way to communicate with a mirai-api-http server.
//!
//! ## MiraiServer
//!
//! First, you should construct a [`MiraiServer`], it contains a [`base_url`] property which is the address to the server.
//!
//! ```rust
//! use mirai::session::MiraiServer;
//!
//! let server = MiraiServer::new("http://localhost:8080");
//! ```
//!
//! You can use [`MiraiServer::about`] function to get the server status.
//!
//! ## Session
//!
//! Second, you can use [`MiraiServer::auth`] to authorize, the auth key can be found in mirai-console output when it starts.
//!
//! ```rust
//! let session = server.auth("auth_key_should_be_kept_secret");
//! ```
//!
//! After authorization, you can bind your session with a bot that is logged in the server.
//!
//! ```rust
//! let account = "QQ Account".parse().unwrap();
//! session.verify(account);
//! ```
//!
//! You can send and get messages now!
//!
//! After these, you should release the connection which your session to a bot.
//!
//! ```rust
//! session.release(account);
//! ```
//!
//! If not, the useless bot will continue to receive messages, this will bring **memory leak**.
//!

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{Result, ImpossibleError, assert};
use crate::{Code, Target};


/// # MiraiServer
///
/// mirai server contains server address ([base_url]).
#[derive(Clone, Debug)]
pub struct MiraiServer {
    pub base_url: String
}

/// # Session
///
/// a session which authorized with a mirai server ([server])
#[derive(Debug)]
pub struct Session {
    pub(crate) client: Client,
    pub server: MiraiServer,
    pub key: String,
}

#[derive(Deserialize)]
pub(crate) struct CommonResponse {
    pub code: Code,
    pub msg: String,
}

#[derive(Deserialize)]
pub struct AboutResponse {
    pub code: Code,
    pub data: AboutData,
}

#[derive(Deserialize)]
pub struct AboutData {
    pub version: String
}

impl MiraiServer {
    pub fn new(base_url: &str) -> MiraiServer {
        MiraiServer {
            base_url: base_url.to_string()
        }
    }

    pub fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    pub async fn about(&self) -> Result<AboutResponse> {
        let resp: AboutResponse = reqwest::get(&self.url("/about"))
            .await?
            .json().await?;

        Ok(resp)
    }

    pub async fn auth(&self, auth_key: &str) -> Result<Session> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "authKey")]
            auth_key: String
        }

        #[derive(Deserialize)]
        struct Response {
            code: Code,
            session: Option<String>,
        }

        let client = Client::new();
        let req = Request {
            auth_key: auth_key.to_string()
        };

        let result: Response = client.post(&self.url("/auth"))
            .json(&req).send().await?
            .json().await?;

        assert(result.code, "Auth")?;

        Ok(Session {
            client,
            server: self.clone(),
            key: result.session.ok_or(ImpossibleError("session is None".to_string()))?,
        })
    }

    pub async fn run_command(&self, auth_key: &str, command: &str, args: &[&str]) -> Result<String> {
        #[serde(rename_all = "camelCase")]
        #[derive(Serialize)]
        struct Request<'s> {
            auth_key: &'s str,
            name: &'s str,
            args: &'s [&'s str],
        }

        let req = Request {
            auth_key,
            name: command,
            args,
        };

        let client = Client::new();
        let text = client.post(&self.url("/command/send"))
            .json(&req).send().await?
            .text().await?;

        Ok(text)
    }
}

impl Session {
    pub fn url(&self, path: &str) -> String {
        self.server.url(path)
    }

    pub async fn verify(&self, qq: Target) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            qq: Target,
        }

        let req = Request {
            session_key: self.key.clone(),
            qq,
        };

        let result: CommonResponse = self.client.post(&self.url("/verify"))
            .json(&req).send().await?
            .json().await?;

        assert(result.code, "Verify")
    }

    pub async fn release(&self, qq: Target) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            qq: Target,
        }

        let req = Request {
            session_key: self.key.clone(),
            qq,
        };

        let resp: CommonResponse = self.client.post(&self.url("/release"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Release")
    }
}