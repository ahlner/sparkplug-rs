#![doc = include_str!("../README.md")]
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
