mod connect;

use connect::connect;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let session = connect(Client::new()).await;

    println!("{:?}", session.get_config().await.unwrap());
    println!("{:?}", session.modify_config(8086, true).await.unwrap());
    println!("{:?}", session.get_config().await.unwrap());
}
