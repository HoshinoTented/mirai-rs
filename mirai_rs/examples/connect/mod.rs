#![allow(dead_code)]

use mirai::session::{MiraiConnection, Session};
use std::time::Duration;
use std::io::stdin;
use reqwest::{Client, Proxy};

pub async fn connect_server(client: Client) -> MiraiConnection {
    let connection = MiraiConnection::new("http://localhost:8080", client);

    loop {
        println!("Try to connecting to server: {}", connection.base_url);

        match connection.about().await {
            Err(e) => {
                println!("Failed, try to reconnect: {:?}", e);
                std::thread::sleep(Duration::from_secs(1));
            }

            Ok(resp) => {
                println!("Success. Mirai Server Version: {}", resp.data().version());
                break;
            }
        }
    }

    connection
}

pub async fn authorize(connection: MiraiConnection) -> Session {
    let mut auth_key = String::new();

    println!("Please input auth key: ");
    stdin().read_line(&mut auth_key).unwrap();

    let session = connection.auth(auth_key.trim()).await.unwrap();
    println!("Authorizing Successful.");

    session
}

pub async fn verifying(session: &Session) {
    let mut id = String::new();

    println!("Please input qq id: ");
    stdin().read_line(&mut id).unwrap();
    session.verify(id.trim().parse().expect("wrong qq id format")).await.unwrap();

    println!("Binding Successful.");
}

pub async fn connect(client: Client) -> Session {
    let connection = connect_server(client).await;
    let session = authorize(connection).await;

    verifying(&session).await;

    session
}

pub fn default_client() -> Client {
    Client::new()
}

pub fn proxy_client() -> Client {
    Client::builder().proxy(Proxy::http("http://localhost:8888").unwrap()).build().unwrap()
}