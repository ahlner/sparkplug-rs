//! Rust protobuf payload for Sparkplug.
//! The structs are generated with the protobuf IDL of the [`Eclipse Tahu`] project. For more information
//! look at the homepage of the [`Eclipse Sparkplug project`].
//!
//! [`Eclipse Tahu`]: https://github.com/eclipse/tahu/blob/master/sparkplug_b/sparkplug_b.proto
//! [`Eclipse Sparkplug project`]: https://sparkplug.eclipse.org
mod topic_namespace;
mod device_message_types;

pub use protobuf;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
pub use sparkplug_b::*;
pub use topic_namespace::*;
pub use device_message_types::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_include() {
        use crate::Payload;
        let p = &Payload::new();
        assert_eq!(0, p.get_timestamp())
    }
}