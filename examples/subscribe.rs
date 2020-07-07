mod connect;

use connect::connect;

use std::sync::{Arc, Mutex};
use std::collections::vec_deque::VecDeque;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

use mirai::message::EventPacket;
use mirai::session::Session;
use mirai::test::Mock;

use reqwest::Client;
use std::time::Duration;

fn start_subscribe<P>(session: Arc<Session>) -> Sender<Arc<EventPacket>> {
    let (tx, _) = broadcast::channel(usize::MAX);

    {
        let _job = tokio::spawn(async move {
            loop {
                let events = session.fetch_newest_message(1).await;

                match events {
                    Ok(events) => {
                        events.into_iter().next().map(|event| {
                            let event = Arc::new(event);
                            tx.send(event.clone()).unwrap();
                        });
                    }

                    Err(e) => println!("{:?}", e)
                }
            }
        });
    }

    tx
}

#[tokio::main]
async fn main() {
    let session = Arc::new(connect(Client::new()).await);
    let tx = start_subscribe(session);
    let rx = tx.subscribe();

    for packet in rx {
        println!("{:?}", packet);
    }
}