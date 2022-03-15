use crate::{DeviceMessageType, NodeMessageType, TopicNamespace};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Rust representation of a sparkplug&trade; MQTT topic-name.
///
/// The [TopicName] can be one of three possible types:
/// - [TopicName::NodeMessage] for Edge Nodes
/// - [TopicName::DeviceMessage] for devices
/// - [TopicName::StateMessage] for SCADA applications
///
/// # Examples
/// ```
/// # use std::str::FromStr;
/// # use sparkplug_rs::{NodeMessageType, TopicName, TopicNamespace};
/// let node = TopicName::new_node_message(TopicNamespace::SPBV1_0,
///                                        "my_group".to_string(),
///                                        NodeMessageType::NBIRTH,
///                                        "nodeId".to_string());
/// assert_eq!(node.to_string(), "spBv1.0/my_group/NBIRTH/nodeId");
///
/// let topic: TopicName = TopicName::from_str("spBv1.0/my_group/NBIRTH/nodeId").unwrap();
/// assert_eq!(topic, node);
/// ```
///
/// ```
/// # use std::str::FromStr;
/// # use sparkplug_rs::{DeviceMessageType, NodeMessageType, TopicName, TopicNamespace};
/// let device = TopicName::new_device_message(TopicNamespace::SPBV1_0,
///                                            "my_group".to_string(),
///                                            DeviceMessageType::DBIRTH,
///                                            "nodeId".to_string(),
///                                            "deviceId".to_string());
/// assert_eq!(device.to_string(), "spBv1.0/my_group/DBIRTH/nodeId/deviceId");
///
/// let topic: TopicName = TopicName::from_str("spBv1.0/my_group/DBIRTH/nodeId/deviceId").unwrap();
/// assert_eq!(topic, device);
/// ```
///
/// ```
/// # use std::str::FromStr;
/// # use sparkplug_rs::{DeviceMessageType, NodeMessageType, TopicName, TopicNamespace};
/// let state = TopicName::new_state_message("scada_host_id".to_string());
/// assert_eq!(state.to_string(), "STATE/scada_host_id");
///
/// let topic: TopicName = TopicName::from_str("STATE/scada_host_id").unwrap();
/// assert_eq!(state, topic);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TopicName {
    /// A message for edge-nodes
    NodeMessage {
        /// The namespace element of the Topic Namespace is the root element that will define
        /// both the structure of the remaining namespace elements as well as the encoding used
        /// for the associated payload data.
        namespace: TopicNamespace,

        /// The group_id element of the Topic Namespace provides for a logical grouping
        /// of MQTT EoN nodes into the MQTT Server and back out to the consuming MQTT Clients.
        group_id: String,

        /// The message_type element of the Topic Namespace provides an indication as to
        /// how to handle the MQTT payload of the message.
        node_message_type: NodeMessageType,

        /// The edge_node_id element of the Sparkplug (TM) Topic Namespace uniquely identifies the MQTT EoN node within the infrastructure.
        edge_node_id: String,
    },

    /// A message for devices
    DeviceMessage {
        /// The namespace element of the Topic Namespace is the root element that will define
        /// both the structure of the remaining namespace elements as well as the encoding used
        /// for the associated payload data.
        namespace: TopicNamespace,

        /// The group_id element of the Topic Namespace provides for a logical grouping
        /// of MQTT EoN nodes into the MQTT Server and back out to the consuming MQTT Clients.
        group_id: String,

        /// The message_type element of the Topic Namespace provides an indication as to
        /// how to handle the MQTT payload of the message.
        device_message_type: DeviceMessageType,

        /// The edge_node_id element of the Sparkplug (TM) Topic Namespace uniquely identifies the MQTT EoN node within the infrastructure.
        edge_node_id: String,

        /// The device_id element of the Sparkplug (TM) Topic Namespace identifies a device attached (physically or logically) to the MQTT EoN node.
        device_id: String,
    },

    /// A state message for scada systems
    StateMessage {
        /// The id of the SCADA application
        scada_host_id: String,
    },
}

impl TopicName {
    /// Constructs a new [TopicName] of type [TopicName::NodeMessage]
    pub const fn new_node_message(
        namespace: TopicNamespace,
        group_id: String,
        node_message_type: NodeMessageType,
        edge_node_id: String,
    ) -> Self {
        TopicName::NodeMessage {
            namespace,
            group_id,
            node_message_type,
            edge_node_id,
        }
    }

    /// Constructs a new [TopicName] of type [TopicName::DeviceMessage]
    pub const fn new_device_message(
        namespace: TopicNamespace,
        group_id: String,
        device_message_type: DeviceMessageType,
        edge_node_id: String,
        device_id: String,
    ) -> Self {
        TopicName::DeviceMessage {
            namespace,
            group_id,
            device_message_type,
            edge_node_id,
            device_id,
        }
    }

    /// Constructs a new [TopicName] of type [TopicName::StateMessage]
    pub const fn new_state_message(scada_host_id: String) -> Self {
        TopicName::StateMessage { scada_host_id }
    }
}

impl FromStr for TopicName {
    type Err = Box<dyn Error + Sync + Send + 'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let mut iter = parts.iter();

            if Some(&"STATE") == iter.next() {
                if let Some(scada_host_id) = iter.next() {
                    return Ok(TopicName::StateMessage {
                        scada_host_id: scada_host_id.to_string(),
                    });
                }
            }
        } else if parts.len() == 4 {
            let mut iter = parts.iter();
            let namespace;
            let group_id;
            let node_message_type;
            let edge_node_id;

            if let Some(s) = iter.next() {
                namespace = TopicNamespace::from_str(s)?;

                if let Some(s) = iter.next() {
                    group_id = s.to_string();

                    if let Some(s) = iter.next() {
                        node_message_type = NodeMessageType::from_str(s)?;

                        if let Some(s) = iter.next() {
                            edge_node_id = s.to_string();

                            return Ok(TopicName::NodeMessage {
                                namespace,
                                group_id,
                                node_message_type,
                                edge_node_id,
                            });
                        }
                    }
                }
            }
        } else if parts.len() == 5 {
            let mut iter = parts.iter();
            let namespace;
            let group_id;
            let device_message_type;
            let edge_node_id;

            if let Some(s) = iter.next() {
                namespace = TopicNamespace::from_str(s)?;

                if let Some(s) = iter.next() {
                    group_id = s.to_string();

                    if let Some(s) = iter.next() {
                        device_message_type = DeviceMessageType::from_str(s)?;

                        if let Some(s) = iter.next() {
                            edge_node_id = s.to_string();

                            if let Some(s) = iter.next() {
                                return Ok(TopicName::DeviceMessage {
                                    namespace,
                                    group_id,
                                    device_message_type,
                                    edge_node_id,
                                    device_id: s.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        Err(Box::from(TopicNameParseError))
    }
}

impl ToString for TopicName {
    fn to_string(&self) -> String {
        match self {
            TopicName::NodeMessage {
                namespace,
                group_id,
                node_message_type,
                edge_node_id,
            } => format!(
                "{}/{}/{}/{}",
                namespace.to_string(),
                group_id,
                node_message_type.to_string(),
                edge_node_id
            ),
            TopicName::DeviceMessage {
                namespace,
                group_id,
                device_message_type,
                edge_node_id,
                device_id,
            } => format!(
                "{}/{}/{}/{}/{}",
                namespace.to_string(),
                group_id,
                device_message_type.to_string(),
                edge_node_id,
                device_id
            ),
            TopicName::StateMessage { scada_host_id } => format!("STATE/{}", scada_host_id),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TopicNameParseError;

impl Display for TopicNameParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TopicNameParseError {}

#[cfg(test)]
mod tests {
    use crate::{DeviceMessageType, NodeMessageType, TopicName, TopicNamespace};
    use std::str::FromStr;

    #[test]
    fn parse_state() {
        TopicName::from_str("STATE/scada_id").unwrap();
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            TopicName::from_str("").unwrap_err().to_string(),
            "TopicNameParseError"
        );
        assert_eq!(
            TopicName::from_str("STATE/to_many/slashes")
                .unwrap_err()
                .to_string(),
            "TopicNameParseError"
        );
        assert_eq!(
            TopicName::from_str("STATE/to_many/slashes/x/x/x")
                .unwrap_err()
                .to_string(),
            "TopicNameParseError"
        );
        assert_eq!(
            TopicName::from_str("wrong_namespace/to_many/slashes/x/x")
                .unwrap_err()
                .to_string(),
            "Error parsing topic's namespace"
        );
        assert_eq!(
            TopicName::from_str("wrong_namespace/to_many/slashes/x")
                .unwrap_err()
                .to_string(),
            "Error parsing topic's namespace"
        );
        assert_eq!(
            TopicName::from_str("wrong_namespace/to_many/slashes/x")
                .unwrap_err()
                .to_string(),
            "Error parsing topic's namespace"
        );
        assert_eq!(
            TopicName::from_str("spBv1.0/my_group/WRONG/nodeId")
                .unwrap_err()
                .to_string(),
            "Unknown NodeMessageType"
        );
        assert_eq!(
            TopicName::from_str("spBv1.0/my_group/WRONG/nodeId/deviceId")
                .unwrap_err()
                .to_string(),
            "Unknown DeviceMessageType"
        );
    }

    #[test]
    fn test_parse_node_cmd() {
        if let TopicName::NodeMessage {
            namespace,
            group_id,
            node_message_type,
            edge_node_id,
        } = TopicName::from_str("spBv1.0/my_group/NBIRTH/nodeId").unwrap()
        {
            assert_eq!(namespace, TopicNamespace::SPBV1_0);
            assert_eq!(group_id, "my_group");
            assert_eq!(node_message_type, NodeMessageType::NBIRTH);
            assert_eq!(edge_node_id, "nodeId");
        } else {
            panic!();
        }
    }

    #[test]
    fn test_parse_device_cmd() {
        if let TopicName::DeviceMessage {
            namespace,
            group_id,
            device_message_type,
            edge_node_id,
            device_id,
        } = TopicName::from_str("spBv1.0/my_group/DBIRTH/nodeId/deviceId").unwrap()
        {
            assert_eq!(namespace, TopicNamespace::SPBV1_0);
            assert_eq!(group_id, "my_group");
            assert_eq!(device_message_type, DeviceMessageType::DBIRTH);
            assert_eq!(edge_node_id, "nodeId");
            assert_eq!(device_id, "deviceId");
        } else {
            panic!();
        }
    }
}
