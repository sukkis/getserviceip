use getserviceip::run;
use std::net::TcpListener; // Adjust the path to your lib.rs module

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bind to a specific address
    let listener = TcpListener::bind("127.0.0.1:8087")?;
    run(listener)?.await
}
