# Deprecated
This was a failed attempt at finding a suitable abstraction. The async-codec crate might be what you need instead.

# Async Serialization

Traits for types that can be asynchronously serialized into AsyncWrites and deserialized from AsyncReads. Unlike serde's approach, the serialized data does not need to be in memory at once, and it saves a step of copying.
