mod connect;

use connect::connect;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel, IntoIter};
use mirai::message::EventPacket;
use mirai::message::event::MessageEvent;
use mirai::session::Session;
use std::iter::Filter;

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

type SubscribeFilter = Filter<IntoIter<EventPacket>, fn(&EventPacket) -> bool>;

fn subscribe_filter(bus: Bus, predicate: fn(&EventPacket) -> bool) -> SubscribeFilter {
    subscribe(bus).into_iter().filter(predicate)
}

fn subscribe_group_message(bus: Bus) -> SubscribeFilter {
    subscribe_filter(bus, |e| {
        if let EventPacket::MessageEvent(MessageEvent::GroupMessage { .. }) = e {
            true
        } else {
            false
        }
    })
}

#[tokio::main]
async fn main() {
    let session = Arc::new(connect().await);
    let bus = init_subscribe(session);

    let rc = subscribe_group_message(bus);

    for packet in rc {
        println!("{:?}", packet);
    }
}