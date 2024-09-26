use getserviceip::{run, IpInfo};
use log::{info, trace, LevelFilter};
use std::net::TcpListener; // Adjust the path to your lib.rs module
use std::sync::{Arc, Mutex};
use systemd_journal_logger::JournalLog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable logging to systemd journal
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(LevelFilter::Info);

    trace!("Start server");
    // Bind to a specific address
    let listener = TcpListener::bind("0.0.0.0:8087")?;
    info!("at address {:?}", &listener);
    let state = Arc::new(Mutex::new(Vec::<IpInfo>::new()));
    run(listener, state)?.await
}
