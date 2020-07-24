mod mock;

use mock::MockServer;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Running...");
    MockServer::new(mirai_test::HOST, mirai_test::PORT).start().await
}