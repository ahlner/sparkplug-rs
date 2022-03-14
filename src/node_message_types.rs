use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Enum for node-message types.
///
/// The string representation for use in MQTT's topic name can produced by [ToString::to_string]
///
/// # Examples
/// ```
/// # use sparkplug_rs::NodeMessageType;
/// assert_eq!(NodeMessageType::NBIRTH.to_string(), "NBIRTH".to_string());
/// ```
///
/// For conversion from the MQTT's topic representation use [FromStr::from_str]
///
/// # Examples
/// ```
/// # use std::str::FromStr;
/// # use sparkplug_rs::NodeMessageType;
/// assert_eq!(NodeMessageType::from_str("NBIRTH").unwrap(), NodeMessageType::NBIRTH);
/// assert!(NodeMessageType::from_str("xyz").is_err());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum NodeMessageType {
    NBIRTH,
    NDEATH,
    NDATA,
    NCMD,
}

impl ToString for NodeMessageType {
    fn to_string(&self) -> String {
        match self {
            NodeMessageType::NBIRTH => "NBIRTH".to_string(),
            NodeMessageType::NDEATH => "NDEATH".to_string(),
            NodeMessageType::NDATA => "NDATA".to_string(),
            NodeMessageType::NCMD => "NCMD".to_string(),
        }
    }
}

pub struct NodeMessageTypeParseError;

impl Debug for NodeMessageTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for NodeMessageTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unknown NodeMessageType")
    }
}

impl Error for NodeMessageTypeParseError {}

impl FromStr for NodeMessageType {
    type Err = NodeMessageTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NBIRTH" => Ok(NodeMessageType::NBIRTH),
            "NDEATH" => Ok(NodeMessageType::NDEATH),
            "NDATA" => Ok(NodeMessageType::NDATA),
            "NCMD" => Ok(NodeMessageType::NCMD),
            _ => Err(NodeMessageTypeParseError),
        }
    }
}
