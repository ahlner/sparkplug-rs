//! Rust protobuf payload for Sparkplug.
//! The structs are generated with the protobuf IDL of the [`Eclipse Tahu`] project. For more information
//! look at the homepage of the [`Eclipse Sparkplug project`].
//!
//! [`Eclipse Tahu`]: https://github.com/eclipse/tahu/blob/master/sparkplug_b/sparkplug_b.proto
//! [`Eclipse Sparkplug project`]: https://sparkplug.eclipse.org
//!
//! # MQTT Topic Names
//!
//! This crate provides a convenient way to handle MQTT topic names with [TopicName].
//!
//! # Examples
//! ```
//! # use std::str::FromStr;
//! # use sparkplug_rs::{NodeMessageType, TopicName, TopicNamespace};
//! let node = TopicName::new_node_message(TopicNamespace::SPBV1_0,
//!                                        "my_group".to_string(),
//!                                        NodeMessageType::NBIRTH,
//!                                        "nodeId".to_string());
//! assert_eq!(node.to_string(), "spBv1.0/my_group/NBIRTH/nodeId");
//!
//! let topic: TopicName = TopicName::from_str("spBv1.0/my_group/NBIRTH/nodeId").unwrap();
//! assert_eq!(topic, node);
//! ```
mod topic_namespace;
pub use topic_namespace::*;

mod device_message_types;
pub use device_message_types::*;

mod node_message_types;
pub use node_message_types::*;

mod topic_name;
pub use topic_name::*;

pub use protobuf;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
pub use sparkplug_b::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_include() {
        use crate::Payload;
        let p = &Payload::new();
        assert_eq!(0, p.get_timestamp())
    }
}
