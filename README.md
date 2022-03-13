# sparkplug-rs

Rust protobuf payload for Sparkplug.
The structs are generated with the protobuf IDL of the [`Eclipse Tahu`] project. For more information
look at the homepage of the [`Eclipse Sparkplug project`].

[`Eclipse Tahu`]: https://github.com/eclipse/tahu/blob/master/sparkplug_b/sparkplug_b.proto
[`Eclipse Sparkplug project`]: https://sparkplug.eclipse.org

## MQTT Topic Names

This crate provides a convenient way to handle MQTT topic names with [TopicName].

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
