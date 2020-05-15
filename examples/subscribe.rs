mod connect;

use connect::connect;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use mirai::message::EventPacket;
use mirai::message::event::MessageEvent;
use mirai::session::Session;
use std::iter::Filter;
use std::ops::{DerefMut, Deref};

type Bus = Arc<Mutex<Vec<Sender<EventPacket>>>>;

struct EventHandler<P> {
    sender: Sender<EventPacket>,
    predicate: P,
}

impl<P> EventHandler<P> {
    pub fn new(predicate: P) -> (EventHandler<P>, Receiver<EventPacket>) {
        let (sc, rc) = channel();

        let handler = EventHandler {
            sender: sc,
            predicate,
        };

        (handler, rc)
    }
}

struct EventBus<P> {
    bus: Vec<EventHandler<P>>
}

impl<P> EventBus<P> {
    pub fn new() -> EventBus<P> {
        EventBus {
            bus: Vec::new()
        }
    }

    pub fn register(&mut self, handler: EventHandler<P>) {
        self.bus.push(handler);
    }

    pub fn subscribe(&mut self, predicate: P) -> Receiver<EventPacket> {
        let (handler, rc) = EventHandler::new(predicate);
        self.register(handler);

        rc
    }
}

fn new_subscribe<P>(session: Arc<Session>) -> Arc<Mutex<EventBus<P>>>
    where P: FnMut(&EventPacket) -> bool + Send + 'static {
    let subscribers = Arc::new(Mutex::new(EventBus::<P>::new()));

    {
        let subscribers = subscribers.clone();
        let _job = tokio::spawn(async move {
            loop {
                let events = session.fetch_newest_message(1).await;

                match events {
                    Ok(events) => {
                        let first = events.into_iter().next();
                        if let Some(event) = first {
                            let mut subscribers = subscribers.lock().unwrap();

                            for subscriber in subscribers.bus.iter_mut() {
                                if (subscriber.predicate)(&event) {
                                    subscriber.sender.send(event.clone());      // TODO: use Result
                                }
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

#[tokio::main]
async fn main() {
    let session = Arc::new(connect().await);
    let bus = new_subscribe::<fn(&EventPacket) -> bool>(session);

    let rc = {
        let mut bus = bus.lock().unwrap();

        bus.subscribe(|event| if let EventPacket::MessageEvent(MessageEvent::FriendMessage { .. }) = event {
            true
        } else {
            false
        })
    };

    for packet in rc {
        println!("{:?}", packet);
    }
}