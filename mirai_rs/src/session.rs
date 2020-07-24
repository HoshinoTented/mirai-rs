//! This mod provides a way to communicate with a mirai-api-http server.
//!
//! ## MiraiConnection
//!
//! First, you should construct a [`MiraiConnection`], it contains a client and a [`base_url`] property which is the address of the server.
//!
//! ```ignore
//! use mirai::session::MiraiConnection;
//! use reqwest::Client;
//!
//! let connection = MiraiConnection::new("http://localhost:8080", Client::new());
//! ```
//!
//! You can use [`MiraiConnection::about`] function to get the server status.
//!
//! ## Session
//!
//! Second, you can use [`MiraiConnection::auth`] to authorizing, the auth key can be found in mirai-console output when it starts.
//!
//! ```ignore
//! let session = connection.auth("auth_key_should_be_kept_secret");
//! ```
//!
//! After authorizing, you can bind your session with a bot that is logged in the server.
//!
//! ```ignore
//! let account = "QQ Account".parse().unwrap();
//! session.verify(account);
//! ```
//!
//! You can send and get messages now!
//!
//! After these, you should release the connection which your session to a bot.
//!
//! ```ignore
//! session.release(account);
//! ```
//!
//! If not, the useless bot will continue to receive messages, this will bring **memory leak**.
//!

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{Result, assert};
use crate::{Code, Target};

#[derive(Clone, Debug)]
pub struct MiraiConnection {
    pub base_url: String,
    pub(crate) client: Client,
}

/// # Session
///
/// Session is a structure that contains a connection with the server which is authorized, and a Auth Key which received from server.
#[derive(Debug)]
pub struct Session {
    pub connection: MiraiConnection,
    pub key: String,
}

/// # CommonResponse
///
/// The most general response from the mirai server, it only contains a state code and a message string.
#[derive(Deserialize)]
pub(crate) struct CommonResponse {
    pub(crate) code: Code,
    pub(crate) msg: String,
}

impl CommonResponse {
    pub fn new(code: Code, msg: String) -> CommonResponse {
        CommonResponse {
            code,
            msg,
        }
    }

    pub fn code(&self) -> Code {
        self.code
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct AboutResponse {
    pub(crate) code: Code,
    pub(crate) data: AboutData,
}

impl AboutResponse {
    pub fn new(code: Code, data: AboutData) -> Self {
        AboutResponse { code, data }
    }

    pub fn code(&self) -> Code {
        self.code
    }

    pub fn data(&self) -> &AboutData {
        &self.data
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct AboutData {
    pub(crate) version: String
}

impl AboutData {
    pub fn new(version: String) -> Self {
        AboutData { version }
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}

impl MiraiConnection {
    /// Constructing a connection with a server address and a mirai client instance.
    pub fn new(base_url: &str, client: Client) -> MiraiConnection {
        MiraiConnection {
            base_url: base_url.to_string(),
            client,
        }
    }

    /// Connecting the base url from this connection and the given string.
    /// Note that this function only simply connects two string, so you must ensure the given string starts with the '/' separator.
    pub fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    /// send a GET request in order to get the information of the mirai server.
    pub async fn about(&self) -> Result<AboutResponse> {
        let resp: AboutResponse = self.client.get(&self.url("/about"))
            .send().await?
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

        let req = Request {
            auth_key: auth_key.to_string()
        };

        let result: Response = self.client.post(&self.url("/auth"))
            .json(&req).send().await?
            .json().await?;

        assert(result.code, "Auth")?;

        Ok(Session {
            connection: self.clone(),
            key: result.session.unwrap()
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

        let text = self.client.post(&self.url("/command/send"))
            .json(&req).send().await?
            .text().await?;

        Ok(text)
    }
}

impl Session {
    /// Connecting the base url from this connection and the given string.
    /// Note that this function only simply connects two string, so you must ensure the given string starts with the '/' separator.
    pub fn url(&self, path: &str) -> String {
        self.connection.url(path)
    }

    /// Return the client of this session
    pub fn client(&self) -> &Client {
        &self.connection.client
    }

    /// Binding the session with the given QQ ID.
    /// Note that one session can only bind with one QQ ID.
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

        let result: CommonResponse = self.client().post(&self.url("/verify"))
            .json(&req).send().await?
            .json().await?;

        assert(result.code, "Verify")
    }

    /// Release the QQ ID which this session bound before.
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

        let resp: CommonResponse = self.client().post(&self.url("/release"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Release")
    }
}