mod connect;

use std::sync::{mpsc, Arc};
use std::time::Duration;

use mirai::message::event::{EventPacket, MessageEvent};
use mirai::message::content::{MessageContent};
use mirai::message::channel::{AsGroupChannel, AsTempChannel};
use mirai::message::{Message};
use mirai::message::element::Permission;

use connect::connect;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let (sc, rc) = mpsc::channel();
    let session = Arc::new(connect(Client::new()).await);

    {
        let session = session.clone();
        let _job = tokio::spawn(async move {
            loop {
                let events = session.fetch_newest_message(1).await;

                match events {
                    Ok(events) => {
                        let first = events.into_iter().next();
                        if let Some(event) = first {
                            sc.send(event).unwrap();
                        }
                    }

                    Err(e) => println!("{:?}", e)
                }
            }
        });
    }

    for mp in rc.iter() {
        if let EventPacket::MessageEvent(MessageEvent::GroupMessage {
                                             message_chain,
                                             sender
                                         }) = &mp {
            let msg = message_chain.iter().fold(String::new(), |msg, elem| {
                if let MessageContent::Plain { text } = elem {
                    msg + text
                } else {
                    msg
                }
            });

            match msg.trim() {
                "Hello" => {
                    session.send_message(sender.group().as_group_channel(), &Message::new(vec!["qwq".into()])).await.unwrap();
                }

                _ => {}
            };
        }

        println!("{:?}", mp);
    }
}