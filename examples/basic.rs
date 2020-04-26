use mirai_rs::session::Session;
use mirai_rs::message::{MessagePackage, SingleMessage, Message};

use std::io::stdin;
use std::thread;
use std::sync::{mpsc, Arc};

fn main() {
    let mut auth_key = String::new();
    let mut id = String::new();

    println!("Please input auth key: ");
    stdin().read_line(&mut auth_key).expect("input error");

    println!("Please input qq id: ");
    stdin().read_line(&mut id).expect("input error");

    let session = Session::auth("http://localhost:8080", auth_key.trim()).unwrap();
    session.verify(id.trim().parse().expect("wrong qq id format")).unwrap();

    println!("Done: {:?}", session);

    let (sc, rc) = mpsc::channel();
    let session = Arc::new(session);
    let ses = session.clone();

    let _job = thread::spawn(move || {
        loop {
            let mps = ses.fetch_newest_message(1);

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

    println!("{:?}", session.friend_list());
    println!("{:?}", session.group_list());

    for mp in rc.iter() {
        if let MessagePackage::Group {
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

            if msg == "Hello" {
                session.send_group_message(Message::new(
                    sender.group.id,
                    &vec![SingleMessage::Plain { text: String::from("Hi!") }])
                ).unwrap();
            }
        }

        println!("{:?}", mp);
    }
}