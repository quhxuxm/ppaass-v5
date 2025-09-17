use serde::{Deserialize, Serialize};

pub mod address;
pub mod connect;
pub mod encryption;
pub mod error;
pub mod handshake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);

impl From<String> for Username {
    fn from(value: String) -> Self {
        Username(value)
    }
}

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(String);

impl From<String> for MessageId {
    fn from(value: String) -> Self {
        MessageId(value)
    }
}

impl From<MessageId> for String {
    fn from(value: MessageId) -> Self {
        value.0
    }
}

impl Default for MessageId {
    fn default() -> Self {
        MessageId(xid::new().to_string())
    }
}
