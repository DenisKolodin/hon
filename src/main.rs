#[macro_use]
extern crate log;
extern crate env_logger;
extern crate permission;
extern crate authorize;
extern crate mould;
extern crate mould_auth;
extern crate mould_file;
extern crate mould_nfd;

mod session;

use authorize::checkers::StringChecker;
use mould::server::{Suite, start};
use mould_auth::TokenService;
use mould_file::FileService;
use mould_nfd::DialogService;

fn main() {
    env_logger::init().unwrap();
    info!("Starting hon server...");

    let builder = session::UserBuilder::new();
    let mut suite = Suite::new(builder);
    let checker = StringChecker::new();
    suite.register("token-service", TokenService::new(checker));
    suite.register("file-service", FileService::new());
    suite.register("dialog-service", DialogService::new());
    start("localhost:44666", suite);
}
