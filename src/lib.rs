//! Traits for types that can be asynchronously serialized into AsyncWrites and deserialized from
//! AsyncReads. Unlike serde's approach, the serialized data does not need to be in memory at once,
//! and it saves a step of copying.
#![deny(missing_docs)]

extern crate futures_core;
extern crate futures_io;

use futures_core::Future;
use futures_io::{AsyncRead, AsyncWrite, Error as FutIoErr};

/// A type whose values can be serialized into an `AsyncWrite`.
pub trait AsyncSerialize<W: AsyncWrite>: Sized {
    /// The future that performs the serialization.
    type SerializeFuture: Future<Item = (W, Self), Error = (W, FutIoErr)>;

    /// Consume a value and a writer to create a `SerializeFuture`.
    fn into_serialize_future(self, writer: W) -> Self::SerializeFuture;
}

/// A type whose values can be serialized by reference into an `AsyncWrite`.
pub trait AsyncSerializeRef<'val, W: AsyncWrite>: Sized {
    /// The future that performs the serialization.
    type SerializeRefFuture: Future<Item = (W), Error = (W, FutIoErr)>;

    /// Take a reference to a value and a writer to create a `SerializeRefFuture`.
    fn serialize_future_ref(&'val self, writer: W) -> Self::SerializeRefFuture;
}

/// A type whose values can be serialized into an `AsyncWrite`, where the exact number of bytes to
/// write can be computed in advance.
pub trait AsyncSerializeLen<W: AsyncWrite>: Sized + AsyncSerialize<W> {
    /// Compute the exact legth of the serialized value in bytes.
    fn serialized_len(&self) -> usize;
}

/// A type whose values can be serialized by reference into an `AsyncWrite`, where the exact number
/// of bytes to write can be computed in advance.
pub trait AsyncSerializeRefLen<'val, W: AsyncWrite>
    : Sized + AsyncSerializeRef<'val, W> {
    /// Compute the exact legth of the serialized value in bytes.
    fn serialized_len(&self) -> usize;
}

/// An error that occured during deserialiation.
pub enum DeserializeError<E> {
    /// An error propagated from the underlying reader.
    ReaderError(FutIoErr),
    /// An error for describing why the read data could not be deserialized into a value.
    DataError(E),
}

/// A type whose values can be deserialized from an `AsyncRead`.
pub trait AsyncDeserialize<R: AsyncRead>: Sized {
    /// The future that performs the deserialization. It yields back ownerhip of the wrapped
    /// reader, the deserialized value, and the number of bytes it read from the reader.
    type DeserializeFuture: Future<Item = (R, Self, usize), Error = (R, DeserializeError<Self::Error>)>;
    /// The error that is emitted when reading invalid data.
    type Error;

    /// Consume a reader to create a `DeserializeFuture`.
    fn deserialize_future(reader: R) -> Self::DeserializeFuture;
}
