mod connect;

use connect::connect;

use std::sync::{Arc};

use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

use mirai::message::EventPacket;
use mirai::session::Session;

use reqwest::Client;

fn start_subscribe(session: Arc<Session>) -> Arc<Sender<Arc<EventPacket>>> {
    let (tx, _) = broadcast::channel(usize::MAX);
    let tx = Arc::new(tx);

    {
        let tx = tx.clone();
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
    let mut rx = tx.subscribe();

    while let Ok(packet) = rx.recv().await {
        println!("{:?}", packet);
    }
}