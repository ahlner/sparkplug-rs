use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Enumerator for Sparkplugs's topic namespace.
///
/// The MQTT-Representation can be creaed with the [ToString::to_string]-method.
/// ```
/// # use sparkplug_rs::TopicNamespace;
/// assert_eq!("spAv1.0".to_string(), TopicNamespace::SPAV1_0.to_string());
/// assert_eq!("spBv1.0".to_string(), TopicNamespace::SPBV1_0.to_string());
/// ```
///
/// The MQTT-String representation can be parsed with [FromStr::from_str].
///
/// ```
/// # use std::str::FromStr;
/// # use sparkplug_rs::TopicNamespace;
/// assert_eq!(TopicNamespace::from_str("spAv1.0").unwrap(), TopicNamespace::SPAV1_0);
/// assert!(TopicNamespace::from_str("xyz").is_err());
/// ```
#[derive(Debug, PartialEq)]
pub enum TopicNamespace {
    // spAv1.0
    SPAV1_0,
    // spBv1.0
    SPBV1_0
}

impl ToString for TopicNamespace {
    fn to_string(&self) -> String {
        match self {
            TopicNamespace::SPAV1_0 => "spAv1.0".to_string(),
            TopicNamespace::SPBV1_0 => "spBv1.0".to_string()
        }
    }
}

pub struct TopicNamespaceParseError;

impl Debug for TopicNamespaceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for TopicNamespaceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Topic parse error")
    }
}

impl Error for TopicNamespaceParseError {}

impl FromStr for TopicNamespace {
    type Err = TopicNamespaceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spAv1.0" => Ok(TopicNamespace::SPAV1_0),
            "spBv1.0" => Ok(TopicNamespace::SPBV1_0),
            _ => Err(TopicNamespaceParseError)
        }
    }
}
