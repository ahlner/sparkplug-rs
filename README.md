sparkplug-rs
![License](https://img.shields.io/crates/l/sparkplug-rs)
[![Latest Version](https://img.shields.io/crates/v/sparkplug-rs.svg)](https://crates.io/crates/sparkplug-rs)
[![Documentation](https://docs.rs/sparkplug-rs/badge.svg)](https://docs.rs/sparkplug-rs)
[![Issues](https://img.shields.io/github/issues/ahlner/sparkplug-rs)](https://github.com/ahlner/sparkplug-rs/issues)

Rust protobuf payload for Sparkplug&trade;.
The structs are generated with the protobuf IDL of the [`Eclipse Tahu`] project. For more information
look at the homepage of the [`Eclipse Sparkplug project`].

[`Eclipse Tahu`]: https://github.com/eclipse/tahu/blob/master/sparkplug_b/sparkplug_b.proto
[`Eclipse Sparkplug project`]: https://sparkplug.eclipse.org

## MQTT Topic Names

This crate provides a convenient way to handle MQTT topic names with [TopicName].

[TopicName]: https://docs.rs/sparkplug-rs/latest/sparkplug_rs/enum.TopicName.html

## Examples
```rust
let node = TopicName::new_node_message(TopicNamespace::SPBV1_0,
                                       "my_group".to_string(),
                                       NodeMessageType::NBIRTH,
                                       "nodeId".to_string());
assert_eq!(node.to_string(), "spBv1.0/my_group/NBIRTH/nodeId");

let topic: TopicName = TopicName::from_str("spBv1.0/my_group/NBIRTH/nodeId").unwrap();
assert_eq!(topic, node);
```

License: EPL-2.0
