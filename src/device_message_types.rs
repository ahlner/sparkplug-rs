use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Enum for device-message types.
///
/// The string representation for use in MQTT's topic name can produced by [ToString::to_string]
///
/// Example:
/// ```
/// # use sparkplug_rs::DeviceMessageType;
/// assert_eq!(DeviceMessageType::DBIRTH.to_string(), "DBIRTH".to_string());
/// ```
///
/// For conversion from the MQTT's topic representation use [FromStr::from_str]
///
/// Example:
/// ```
/// # use std::str::FromStr;
/// # use sparkplug_rs::DeviceMessageType;
/// assert_eq!(DeviceMessageType::from_str("DBIRTH").unwrap(), DeviceMessageType::DBIRTH);
/// assert!(DeviceMessageType::from_str("xyz").is_err());
/// ```
#[derive(Debug, PartialEq)]
pub enum DeviceMessageType {
    DBIRTH,
    DDEATH,
    DDATA,
    DCMD
}

impl ToString for DeviceMessageType {
    fn to_string(&self) -> String {
        match self {
            DeviceMessageType::DBIRTH => "DBIRTH".to_string(),
            DeviceMessageType::DDEATH=> "DDEATH".to_string(),
            DeviceMessageType::DDATA=> "DDATA".to_string(),
            DeviceMessageType::DCMD=> "DCMD".to_string()
        }
    }
}

pub struct DeviceMessageTypeParseError;

impl Debug for DeviceMessageTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for DeviceMessageTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unknown DeviceMessageType")
    }
}

impl Error for DeviceMessageTypeParseError {}

impl FromStr for DeviceMessageType {
    type Err = DeviceMessageTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DBIRTH" => Ok(DeviceMessageType::DBIRTH),
            "DDEATH" => Ok(DeviceMessageType::DDEATH),
            "DDATA" => Ok(DeviceMessageType::DDATA),
            "DCMD" => Ok(DeviceMessageType::DCMD),
            _ => Err(DeviceMessageTypeParseError)
        }
    }
}