use getserviceip::{run, IpInfo};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub fn spawn() -> (String, Arc<Mutex<Vec<IpInfo>>>) {
    // Bind to a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // Create the server
    let state = Arc::new(Mutex::new(Vec::<IpInfo>::new()));
    let server = run(listener, state.clone()).expect("Failed to bind address");

    // Create the server
    //let server = run(listener).expect("Failed to bind address");

    // Spawn the server in a separate thread
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);

    // Return the address of the server
    (format!("http://127.0.0.1:{}", port), state)
}
