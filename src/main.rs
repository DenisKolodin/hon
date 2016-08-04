#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mould;

use mould::{Session, Builder};
use mould::server::{Suite, start};

pub struct UserSession { }

impl Session for UserSession { }

struct UserBuilder { }

impl UserBuilder {
    fn new() -> Self {
        UserBuilder { }
    }
}

impl Builder<UserSession> for UserBuilder {
    fn build(&self) -> UserSession {
        UserSession { }
    }
}

fn main() {
    env_logger::init().unwrap();
    info!("Starting hon server...");

    let builder = UserBuilder::new();
    let suite = Suite::new(builder);
    start("localhost:44666", suite);
}
