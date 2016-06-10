#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mould;

use mould::server::{ServicesMap, start};

mod session {
    use mould::session::SessionData;

    pub struct UserSessionData { }

    impl SessionData for UserSessionData { }

    impl Default for UserSessionData {
        fn default() -> Self {
            UserSessionData { }
        }
    }
}

fn main() {
    env_logger::init().unwrap();
    info!("Starting hon server...");

    let services: ServicesMap<session::UserSessionData> = ServicesMap::new();
    start("localhost:44666", services);
}
