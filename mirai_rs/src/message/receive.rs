//! You can use some function in this mod to receive message or event with a session

use serde::Deserialize;

use crate::Code;
use crate::session::Session;
use crate::message::event::EventPacket;
use crate::error::{assert, HttpResult};

impl Session {
    async fn get_message(&self, is_fetch: bool, is_newest: bool, count: usize) -> HttpResult<Vec<EventPacket>> {
        #[derive(Deserialize)]
        struct Response {
            code: Code,
            data: Vec<EventPacket>,
        }

        let url = format!("/{is_fetch}{is_newest}Message?sessionKey={sessionKey}&count={count}",
                          is_fetch = if is_fetch { "fetch" } else { "peek" },
                          is_newest = if is_newest { "Latest" } else { "" },
                          sessionKey = self.key,
                          count = count);

        let response: Response = self.client().get(&self.url(&url))
            .send().await?
            .json().await?;

        assert(response.code, if is_fetch { "Fetching" } else { "Peeking" })?;

        Ok(response.data)
    }

    pub async fn fetch_newest_message(&self, count: usize) -> HttpResult<Vec<EventPacket>> {
        self.get_message(true, true, count).await
    }

    pub async fn fetch_message(&self, count: usize) -> HttpResult<Vec<EventPacket>> {
        self.get_message(true, false, count).await
    }

    pub async fn peek_newest_message(&self, count: usize) -> HttpResult<Vec<EventPacket>> {
        self.get_message(false, true, count).await
    }

    pub async fn peek_message(&self, count: usize) -> HttpResult<Vec<EventPacket>> {
        self.get_message(false, false, count).await
    }
}