use mirai::session::{MiraiConnection, Session};
use std::time::Duration;
use std::io::stdin;
use reqwest::Client;

pub async fn connect(client: Client) -> Session {
    let server = MiraiConnection::new("http://localhost:8080", client);

    loop {
        println!("Try to connecting to server: {}", server.base_url);

        match server.about().await {
            Err(e) => {
                println!("Failed, try to reconnect: {:?}", e);
                std::thread::sleep(Duration::from_secs(1));
            }

            Ok(resp) => {
                println!("Success. Mirai Server Version: {}", resp.data.version);
                break;
            }
        }
    }


    let mut auth_key = String::new();
    let mut id = String::new();

    println!("Please input auth key: ");
    stdin().read_line(&mut auth_key).unwrap();
    let session = server.auth(auth_key.trim()).await.unwrap();
    println!("Authorizing Successful.");

    println!("Please input qq id: ");
    stdin().read_line(&mut id).unwrap();
    session.verify(id.trim().parse().expect("wrong qq id format")).await.unwrap();

    println!("Binding Successful.");

    session
}