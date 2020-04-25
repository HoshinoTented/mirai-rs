use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::error::{Result, ImpossibleError, assert};
use super::{Code, Target};

#[derive(Debug)]
pub struct Session {
    pub(crate) client: Client,
    pub base_url: String,
    pub key: String,
}

impl Session {
    pub fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    pub fn auth(base_url: &str, auth_key: &str) -> Result<Session> {
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

        let result: Response = client.post(&format!("{}/auth", base_url))
            .json(&req)
            .send()?
            .json()?;

        assert(result.code, "Auth")?;

        Ok(Session {
            client,
            base_url: base_url.to_string(),
            key: result.session.ok_or(ImpossibleError("session is None".to_string()))?,
        })
    }

    pub fn verify(&self, qq: Target) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            qq: Target,
        }

        #[derive(Deserialize)]
        struct Response {
            code: Code,
            msg: String,
        }

        let req = Request {
            session_key: self.key.clone(),
            qq,
        };

        let result: Response = self.client.post(&self.url("/verify"))
            .json(&req)
            .send()?
            .json()?;

        assert(result.code, "Verify")
    }

    pub fn release(&self, qq: Target) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            qq: Target,
        }

        #[derive(Deserialize)]
        struct Response {
            code: Code,
            msg: String,
        }

        let req = Request {
            session_key: self.key.clone(),
            qq,
        };

        let resp: Response = self.client.post(&self.url("/release"))
            .json(&req)
            .send()?
            .json()?;

        assert(resp.code, "Release")
    }
}