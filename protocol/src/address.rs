use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifyAddress {
    IP(SocketAddr),
    Domain(String),
}

impl From<SocketAddr> for UnifyAddress {
    fn from(addr: SocketAddr) -> Self {
        UnifyAddress::IP(addr)
    }
}

impl From<String> for UnifyAddress {
    fn from(addr: String) -> Self {
        UnifyAddress::Domain(addr)
    }
}

impl TryFrom<UnifyAddress> for SocketAddr {
    type Error = Error;
    fn try_from(value: UnifyAddress) -> Result<Self, Self::Error> {
        match value {
            UnifyAddress::IP(addr) => Ok(addr),
            UnifyAddress::Domain(domain_name) => Ok(SocketAddr::from_str(&domain_name)?),
        }
    }
}
