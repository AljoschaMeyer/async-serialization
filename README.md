# Async Serialization

Traits for types that can be asynchronously serialized into AsyncWrites and deserialized from AsyncReads. Unlike serde's approach, the serialized data does not need to be in memory at once, and it saves a step of copying.

# Problems
- this allows only one serialization format per type
- would need to provide implementations for primitive types here, fixing their serialization format

So don't use this :(
