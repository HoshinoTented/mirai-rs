mod connect;

use connect::connect;
use mirai::image::{ImageType};
use mirai::message::{SingleMessage};
use mirai::message::channel::AsGroupChannel;
use reqwest::{Client};

#[tokio::main]
async fn main() {
    const URL: &'static str = "https://avatars3.githubusercontent.com/u/25280943";
    const GROUP: u64 = 972342866;

    let session = connect(Client::new()).await;
    let img = reqwest::get(URL).await.unwrap()
        .bytes().await.unwrap();

    let img = session.upload_image(ImageType::Group, img, String::from("test.png")).await.unwrap();

    println!("Uploaded.");

    session.send_message(GROUP.as_group_channel(), &SingleMessage::from(img).into()).await.unwrap();
}