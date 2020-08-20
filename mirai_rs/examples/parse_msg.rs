mod connect;

use connect::*;
use mirai::message::EventPacket;
use mirai::message::element::Reply;
use mirai::message::event::MessageEvent;
use mirai::message::channel::AsGroupChannel;

#[tokio::main]
async fn main() {
    // let session = connect(default_client()).await;
    //
    // while let Ok(msg) = session.fetch_newest_message(1).await {
    //     if let Some(msg) = msg.first() {
    //         match msg {
    //             EventPacket::MessageEvent(MessageEvent::GroupMessage { message_chain: _, sender }) => {
    //                 // SO HEAVY!
    //                 session.send_message(sender.group().as_group_channel(), &parse_msg(format!(r#"{at} Hello, world! {at}"#, at = sender.at().to_string())).unwrap()).await.unwrap();
    //             }
    //
    //             _ => {}
    //         };
    //
    //         ()
    //     }
    // }
}