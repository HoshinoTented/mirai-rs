use mirai_rs::session::Session;

use std::io::stdin;

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

    loop {
        let mps = session.fetch_newest_message(1);

        match mps {
            Ok(mps) => {
                if mps.len() != 0 {
                    for msg in mps.iter() {
                        println!("{:?}", msg);
                    }
                }
            }

            Err(e) => println!("{:?}", e)
        }
    }
}