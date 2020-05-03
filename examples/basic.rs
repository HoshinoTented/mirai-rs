use std::io::stdin;
use std::sync::{mpsc, Arc};
use std::time::Duration;

use mirai::session::MiraiServer;
use mirai::message::event::{EventPacket, MessageEvent};
use mirai::message::single::SingleMessage;
use mirai::message::channel::{AsGroupChannel, AsTempChannel};
use mirai::message::MessageBuilder;
use mirai::message::element::Permission;

#[tokio::main]
async fn main() {
    let server = MiraiServer::new("http://localhost:8080");

    loop {
        match server.about().await {
            Err(_) => {
                println!("Cannot connect to server, try to reconnect...");
                std::thread::sleep(Duration::from_secs(1));
            }

            Ok(resp) => {
                println!("Mirai Server Version: {}", resp.data.version);
                break;
            }
        }
    }

    let mut auth_key = String::new();
    let mut id = String::new();

    println!("Please input auth key: ");
    stdin().read_line(&mut auth_key).expect("input error");
    let session = server.auth(auth_key.trim()).await.unwrap();
    println!("Authorize Successful.");

    println!("Please input qq id: ");
    stdin().read_line(&mut id).expect("input error");
    session.verify(id.trim().parse().expect("wrong qq id format")).await.unwrap();

    println!("Done: {:?}", session);

    let (sc, rc) = mpsc::channel();
    let session = Arc::new(session);

    {
        let session = session.clone();
        let _job = tokio::spawn(async move {
            loop {
                let mps = session.fetch_newest_message(1).await;

                match mps {
                    Ok(mps) => {
                        let first = mps.into_iter().next();
                        if let Some(mp) = first {
                            sc.send(mp).unwrap();
                        }
                    }

                    Err(e) => println!("{:?}", e)
                }
            }
        });
    }

    println!("{:?}", session.friend_list().await);
    println!("{:?}", session.group_list().await);

    for mp in rc.iter() {
        if let EventPacket::MessageEvent(MessageEvent::GroupMessage {
                                             message_chain,
                                             sender
                                         }) = &mp {
            let msg = message_chain.iter().fold(String::new(), |msg, elem| {
                if let SingleMessage::Plain { text } = elem {
                    msg + text
                } else {
                    msg
                }
            });

            match msg.trim() {
                "Hello" => {
                    session.send_message(&sender.group().as_group_channel(), &MessageBuilder::new()
                        .append_message(SingleMessage::Image { image_id: None, url: None, path: Some("nya.png".to_string()) })
                        .build().unwrap(),
                    ).await.unwrap();
                }

                "mute me" => {
                    if let Permission::Administrator | Permission::Owner = sender.group().permission() {
                        if let Permission::Administrator | Permission::Owner = sender.permission() {
                            session.send_message(&sender.group().as_group_channel(),
                                                 &MessageBuilder::new()
                                                     .append_message("You are too powerful to mute.".into())
                                                     .build().unwrap(),
                            ).await.unwrap();
                        } else {
                            session.mute(sender.group().id(), sender.id(), 60 * 10).await.unwrap();

                            {
                                let session = session.clone();
                                let sender = sender.clone();
                                tokio::spawn(async move {
                                    std::thread::sleep(Duration::from_secs(10));
                                    session.unmute(sender.group().id(), sender.id()).await.unwrap();
                                });
                            }
                        }
                    } else {
                        session.send_message(&sender.group().as_group_channel(),
                                             &MessageBuilder::new()
                                                 .append_message("I have not enough permission QAQ.".into())
                                                 .build().unwrap(),
                        ).await.unwrap();
                    }
                }

                "talk with me" => {
                    session.send_message(&sender.as_temp_channel(),
                                         &MessageBuilder::new()
                                             .append_message("Hello".into())
                                             .build().unwrap(),
                    ).await.unwrap();
                }
                _ => {}
            };

            let config = session.get_group_config(sender.group().id()).await.unwrap();
            let info = session.get_member_info(sender.group().id(), sender.id()).await.unwrap();

            println!("{:?}", config);
            println!("{:?}", info);
        }

        println!("{:?}", mp);
    }
}