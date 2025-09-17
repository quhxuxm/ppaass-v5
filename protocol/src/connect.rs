use crate::address::UnifyAddress;
use crate::error::Error;
use crate::{MessageId, Username};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConnectMessage {
    message_id: MessageId,
    username: Username,
    destination: UnifyAddress,
}

impl TryFrom<AgentConnectMessage> for Vec<u8> {
    type Error = Error;
    fn try_from(value: AgentConnectMessage) -> Result<Self, Self::Error> {
        bincode::serde::encode_to_vec(&value, bincode::config::standard()).map_err(Into::into)
    }
}

impl TryFrom<Vec<u8>> for AgentConnectMessage {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (obj, size) = bincode::serde::decode_from_slice(&value, bincode::config::standard())?;
        debug!(size, "decode agent connect message");
        Ok(obj)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyConnectStatus {
    Success,
    Failed(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConnectMessage {
    message_id: MessageId,
    username: Username,
    destination: UnifyAddress,
    status: ProxyConnectStatus,
}
impl TryFrom<ProxyConnectMessage> for Vec<u8> {
    type Error = Error;
    fn try_from(value: ProxyConnectMessage) -> Result<Self, Self::Error> {
        bincode::serde::encode_to_vec(&value, bincode::config::standard()).map_err(Into::into)
    }
}

impl TryFrom<Vec<u8>> for ProxyConnectMessage {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (obj, size) = bincode::serde::decode_from_slice(&value, bincode::config::standard())?;
        debug!(size, "decode proxy connect message");
        Ok(obj)
    }
}
