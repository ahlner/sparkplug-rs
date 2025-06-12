sparkplug-rs
![License](https://img.shields.io/crates/l/sparkplug-rs)
[![Latest Version](https://img.shields.io/crates/v/sparkplug-rs.svg)](https://crates.io/crates/sparkplug-rs)
[![Documentation](https://docs.rs/sparkplug-rs/badge.svg)](https://docs.rs/sparkplug-rs)
[![Issues](https://img.shields.io/github/issues/ahlner/sparkplug-rs)](https://github.com/ahlner/sparkplug-rs/issues)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fahlner%2Fsparkplug-rs.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fahlner%2Fsparkplug-rs?ref=badge_shield)

Rust protobuf payload for Sparkplug&trade;.
The structs are generated with the protobuf IDL of the [`Eclipse Tahu`] project. For more information
look at the homepage of the [`Eclipse Sparkplug project`].

[`Eclipse Tahu`]: https://github.com/eclipse/tahu/blob/master/sparkplug_b/sparkplug_b.proto

[`Eclipse Sparkplug project`]: https://sparkplug.eclipse.org

## MQTT Topic Names

This crate provides a convenient way to handle MQTT topic names with [TopicName].

[TopicName]: https://docs.rs/sparkplug-rs/latest/sparkplug_rs/enum.TopicName.html

## JSON

For JSON serialization/deserialization of protobuf messages, instead of adding a dependency on `serde`,
I recommend checking out the [`protobuf-json-mapping`](https://crates.io/crates/protobuf-json-mapping) crate.
This crate is specifically designed to work with protobuf messages and provides the JSON mapping functionality you likely need, while keeping dependencies minimal and aligned with
the protobuf ecosystem.
This approach is more lightweight since it avoids bringing in the full `serde` ecosystem when you only need protobuf-specific JSON handling.

## Examples

```rust
# use std::str::FromStr;
# use sparkplug_rs::{NodeMessageType, TopicName, TopicNamespace};
let node = TopicName::new_node_message(TopicNamespace::SPBV1_0,
"my_group".to_string(),
NodeMessageType::NBIRTH,
"nodeId".to_string());
assert_eq!(node.to_string(), "spBv1.0/my_group/NBIRTH/nodeId");

let topic: TopicName = TopicName::from_str("spBv1.0/my_group/NBIRTH/nodeId").unwrap();
assert_eq!(topic, node);
```

License: EPL-2.0

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fahlner%2Fsparkplug-rs.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fahlner%2Fsparkplug-rs?ref=badge_large)