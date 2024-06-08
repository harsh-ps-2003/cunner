Using libp2p's pubsub system for efficient message distribution instead of creating custom messaging channels.


You define your data structures in a .proto file, which is a way to specify structured data. This file format is language-neutral and platform-neutral, designed by Google.
 For each message in your .proto file, prost generates a Rust struct with fields and methods that correspond to the message fields in the .proto file. These structs will include serialization and deserialization capabilities, allowing them to be easily converted to and from the binary format expected by Protocol Buffers.