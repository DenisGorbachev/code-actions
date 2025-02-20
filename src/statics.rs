use crates_io_api::SyncClient;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CRATES_IO_CLIENT: SyncClient = SyncClient::new("code_actions crate", std::time::Duration::from_millis(1000)).unwrap();
}
