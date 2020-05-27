mod connect;

use connect::connect;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use mirai::message::EventPacket;
use mirai::session::Session;
use std::collections::vec_deque::VecDeque;
use reqwest::Client;

struct EventHandler<P> {
    sender: Sender<Arc<EventPacket>>,
    predicate: P,
}

impl<P> EventHandler<P> {
    pub fn new(sc: Sender<Arc<EventPacket>>, predicate: P) -> EventHandler<P> {
        EventHandler {
            sender: sc,
            predicate,
        }
    }
}

struct EventBus<P> {
    bus: VecDeque<EventHandler<P>>
}

impl<P> EventBus<P> {
    pub fn new() -> EventBus<P> {
        EventBus {
            bus: VecDeque::new()
        }
    }

    pub fn register(&mut self, handler: EventHandler<P>) {
        self.bus.push_back(handler)
    }

    pub fn subscribe(&mut self, predicate: P) -> Receiver<Arc<EventPacket>> {
        let (sc, rc) = channel();
        let handler = EventHandler::new(sc, predicate);

        self.register(handler);

        rc
    }
}

fn new_subscribe<P>(session: Arc<Session>) -> Arc<Mutex<EventBus<P>>>
    where P: FnMut(Arc<EventPacket>) -> bool + Send + 'static {
    let subscribers = Arc::new(Mutex::new(EventBus::<P>::new()));

    {
        let subscribers = subscribers.clone();
        let _job = tokio::spawn(async move {
            loop {
                let events = session.fetch_newest_message(1).await;

                match events {
                    Ok(events) => {
                        events.into_iter().next().map(|event| {
                            let mut subscribers = subscribers.lock().unwrap();
                            let mut handler_queue = VecDeque::with_capacity(subscribers.bus.len());
                            let event = Arc::new(event);

                            while let Some(mut handler) = subscribers.bus.pop_front() {
                                if (handler.predicate)(event.clone()) {
                                    if let Ok(_) = handler.sender.send(event.clone()) {
                                        handler_queue.push_back(handler);
                                    } else {
                                        println!("Sending failed");
                                    }
                                }
                            }

                            subscribers.bus.append(&mut handler_queue);
                        });
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
    let session = Arc::new(connect(Client::new()).await);
    let bus = new_subscribe::<fn(Arc<EventPacket>) -> bool>(session);

    let rc = {
        let mut bus = bus.lock().unwrap();

        bus.subscribe(|event| event.is_message())
    };

    for packet in rc {
        println!("{:?}", packet);
    }
}