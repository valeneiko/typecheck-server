#[cfg(windows)]
pub const EOL_LENGTH: usize = 2;
#[cfg(not(windows))]
pub const EOL_LENGTH: usize = 1;

pub mod tsserver;

pub use tsserver::protocol_error::ProtocolError;
