use crate::encryption::Encryption;
use crate::error::Error;
use crate::{MessageId, Username};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandshakeMessage {
    message_id: MessageId,
    username: Username,
    encryption: Encryption,
}

impl TryFrom<AgentHandshakeMessage> for Vec<u8> {
    type Error = Error;
    fn try_from(value: AgentHandshakeMessage) -> Result<Self, Self::Error> {
        bincode::serde::encode_to_vec(&value, bincode::config::standard()).map_err(Into::into)
    }
}

impl TryFrom<Vec<u8>> for AgentHandshakeMessage {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (obj, size) = bincode::serde::decode_from_slice(&value, bincode::config::standard())?;
        debug!(size, "decode agent handshake message");
        Ok(obj)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyHandshakeMessage {
    message_id: MessageId,
    username: Username,
    encryption: Encryption,
}

impl TryFrom<ProxyHandshakeMessage> for Vec<u8> {
    type Error = Error;
    fn try_from(value: ProxyHandshakeMessage) -> Result<Self, Self::Error> {
        bincode::serde::encode_to_vec(&value, bincode::config::standard()).map_err(Into::into)
    }
}

impl TryFrom<Vec<u8>> for ProxyHandshakeMessage {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (obj, size) = bincode::serde::decode_from_slice(&value, bincode::config::standard())?;
        debug!(size, "decode proxy handshake message");
        Ok(obj)
    }
}
