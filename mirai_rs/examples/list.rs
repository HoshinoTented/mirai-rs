mod connect;

use connect::connect;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let session = connect(Client::new()).await;

    println!("{:?}", session.friend_list().await.unwrap());
    println!("{:?}", session.group_list().await.unwrap());
    println!("{:?}", session.group_member_list(972342866).await.unwrap());
}