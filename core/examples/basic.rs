use mirai::session::MiraiServer;
use mirai::message::{MessagePackage, SingleMessage, Message, Permission, MessageBuilder, CanBuildMessage};

use std::io::stdin;
use std::sync::{mpsc, Arc};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let server = MiraiServer::new("http://localhost:8080");
    let mut auth_key = String::new();
    let mut id = String::new();

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
                        let first = mps.get(0);
                        if let Some(mp) = first {
                            sc.send(mp.clone()).unwrap();
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
        if let MessagePackage::GroupMessage {
            message_chain,
            sender
        } = &mp {
            let msg = message_chain.iter().fold(String::new(), |msg, elem| {
                if let SingleMessage::Plain { text } = elem {
                    msg + text
                } else {
                    msg
                }
            });

            match msg.trim() {
                "Hello" => {
                    session.send_group_message(&Message::new(
                        sender.group.id, None,
                        &vec![SingleMessage::Image { image_id: None, url: None, path: Some("nya.png".to_string()) }])
                    ).await.unwrap();
                }

                "mute me" => {
                    if let Permission::Administrator | Permission::Owner = sender.group.permission {
                        if let Permission::Administrator | Permission::Owner = sender.permission {
                            session.send_group_message(
                                &sender.group.build_message()
                                    .append_message("You are too powerful to mute.".into())
                                    .build().unwrap()
                            ).await.unwrap();
                        } else {
                            session.mute(sender.group.id, sender.id, 60 * 10).await.unwrap();

                            {
                                let session = session.clone();
                                let sender = sender.clone();
                                tokio::spawn(async move {
                                    std::thread::sleep(Duration::from_secs(10));
                                    session.unmute(sender.group.id, sender.id).await.unwrap();
                                });
                            }
                        }
                    } else {
                        session.send_group_message(
                            &sender.group.build_message()
                                .append_message("I have not enough permission QAQ.".into())
                                .build().unwrap()
                        ).await.unwrap();
                    }
                }

                "talk with me" => {
                    session.send_temp_message(
                        sender.group.id,
                        &MessageBuilder::new()
                            .target(sender.id)
                            .append_message("Hello".into())
                            .build().unwrap(),
                    ).await.unwrap();
                }
                _ => {}
            };

            let config = session.get_group_config(sender.group.id).await.unwrap();
            let info = session.get_member_info(sender.group.id, sender.id).await.unwrap();

            println!("{:?}", config);
            println!("{:?}", info);
        }

        println!("{:?}", mp);
    }
}