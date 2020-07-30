#![cfg(test)]
#![allow(dead_code)]

use mirai::session::MiraiConnection;
use reqwest::Client;

use crate::{PORT, HOST};
use crate::data::session::About;

fn connection() -> MiraiConnection {
    MiraiConnection::new(&format!("http://{}:{}", HOST, PORT), Client::new())
}

// #[tokio::test]
async fn about() {
    assert_eq!(About::response(), connection().about().await.unwrap());
}