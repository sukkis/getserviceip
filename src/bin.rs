use getserviceip::{run, IpInfo};
use std::net::TcpListener; // Adjust the path to your lib.rs module
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bind to a specific address
    let listener = TcpListener::bind("127.0.0.1:8087")?;
    let state = Arc::new(Mutex::new(Vec::<IpInfo>::new()));
    run(listener, state)?.await
}
