mod connection;
mod types;

pub use connection::Connection;
pub use types::ConnectionLifecycle;
pub use types::DeserializeError;
pub use types::Deserializer;
pub use types::Serializer;

pub(crate) fn interrupted(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::Interrupted
}

pub(crate) fn would_block(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::WouldBlock
}
