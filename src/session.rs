use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::error::{Result, ImpossibleError, assert};
use super::{Code, Target};

#[derive(Debug)]
pub struct Session {
    pub(crate) client: Client,
    pub base_url: String,
    pub key: String,
}

#[derive(Deserialize)]
pub(crate) struct CommonResponse {
    pub code: Code,
    pub msg: String,
}

impl Session {
    pub fn url(&self, path: &str) -> String {
        self.base_url.clone() + path
    }

    pub async fn auth(base_url: &str, auth_key: &str) -> Result<Session> {
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
            .send()
            .await?
            .json()
            .await?;

        assert(result.code, "Auth")?;

        Ok(Session {
            client,
            base_url: base_url.to_string(),
            key: result.session.ok_or(ImpossibleError("session is None".to_string()))?,
        })
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
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

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
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        assert(resp.code, "Release")
    }
}