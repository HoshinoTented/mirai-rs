mod connect;

use connect::{default_client, connect};
use reqwest::{Client, Proxy};
use mirai::config::Config;

#[tokio::main]
async fn main() {
    let session = connect(default_client()).await;

    println!("{:?}", session.get_config().await.unwrap());
    println!("{:?}", session.modify_config(Config {
        cache_size: 8086,
        enable_websocket: true,
    }).await.unwrap());
    println!("{:?}", session.get_config().await.unwrap());
}
