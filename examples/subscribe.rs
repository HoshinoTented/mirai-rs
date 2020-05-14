mod connect;

use connect::connect;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use mirai::message::EventPacket;
use mirai::session::Session;

type Bus = Arc<Mutex<Vec<Sender<EventPacket>>>>;

fn init_subscribe(session: Arc<Session>) -> Bus {
    let subscribers: Bus = Arc::new(Mutex::new(Vec::new()));

    {
        let subscribers = subscribers.clone();
        let _job = tokio::spawn(async move {
            loop {
                let mps = session.fetch_newest_message(1).await;

                match mps {
                    Ok(mps) => {
                        let first = mps.into_iter().next();
                        if let Some(mp) = first {
                            let subscribers = subscribers.lock().unwrap();

                            for subscriber in subscribers.iter() {
                                subscriber.send(mp.clone());            // TODO: use Result
                            }
                        }
                    }

                    Err(e) => println!("{:?}", e)
                }
            }
        });
    }

    subscribers
}

fn subscribe(bus: Bus) -> Receiver<EventPacket> {
    let (sc, rc) = channel();

    let mut bus = bus.lock().unwrap();

    bus.push(sc);

    rc
}

#[tokio::main]
async fn main() {
    let session = Arc::new(connect().await);
    let bus = init_subscribe(session);

    let rc = subscribe(bus);

    for packet in rc.iter() {
        println!("{:?}", packet);
    }
}