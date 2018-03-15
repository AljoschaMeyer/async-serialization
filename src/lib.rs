//! Traits for types that can be asynchronously serialized into AsyncWrites and deserialized from
//! AsyncReads. Unlike serde's approach, the serialized data does not need to be in memory at once,
//! and it saves a step of copying.
#![deny(missing_docs)]

#![deprecated="This was a failed attempt at finding a suitable abstraction. The async-codec crate might be what you need instead."]

extern crate futures_core;
extern crate futures_io;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use futures_core::Future;
use futures_io::{AsyncRead, AsyncWrite, Error as FutIoErr};

/// Base trait for futures that write things into `AsyncWrite`s.
///
/// The future must yield a previously wrapped `AsyncWrite`, and the number of written bytes.
/// If there's an error upon writing, the wrapped `AsyncWrite` is emitted together with the error.
pub trait AsyncWriterFuture<W: AsyncWrite>
    : Future<Item = (W, usize), Error = (W, FutIoErr)> {
    /// Return how many bytes have already been written.
    fn already_written(&self) -> usize;
}

/// Base trait for futures that write things into `AsyncWrite`s and can precompute the exact number
/// of bytes to write.
pub trait AsyncWriterFutureLen<W: AsyncWrite>: AsyncWriterFuture<W> {
    /// Compute the exact number of bytes that will still be written by this future.
    fn remaining_bytes(&self) -> usize;
}

/// A future that asynchronously serializes something into a wrapped AsyncWrite and then returns
/// the wrapped AsyncWrite and how many bytes were written.
pub trait AsyncSerialize<W: AsyncWrite>: AsyncWriterFuture<W> {
    /// The type of values serialized.
    type Serialized;

    /// Create a new instance, consuming the value to serialize and wrapping the `AsyncWrite` to
    /// serialize into.
    fn from_val(writer: W, val: Self::Serialized) -> Self;
}

/// An `AsyncSerialize` that can precompute the exact number of bytes to write.
pub trait AsyncSerializeLen<W: AsyncWrite>
    : AsyncSerialize<W> + AsyncWriterFutureLen<W> {
    /// Compute the exact number of bytes that would be written in total if the given value was
    /// serialized.
    fn total_bytes(val: &Self::Serialized) -> usize;
}

/// A future that asynchronously serializes something by reference into a wrapped AsyncWrite.
pub trait AsyncSerializeRef<'val, W: AsyncWrite>: AsyncWriterFuture<W> {
    /// The type of values serialized.
    type Serialized;

    /// Create a new instance, taking a reference to the value to serialize and wrapping the
    /// `AsyncWrite` to serialize into.
    fn from_ref(writer: W, val: &'val Self::Serialized) -> Self;
}

/// An `AsyncSerializeRef` that can precompute the exact number of bytes to write.
pub trait AsyncSerializeRefLen<'val, W: AsyncWrite>
    : AsyncSerializeRef<'val, W> + AsyncWriterFutureLen<W> {
    /// Compute the exact number of bytes that would be written in total if the given value was
    /// serialized.
    fn total_bytes(val: &Self::Serialized) -> usize;
}

/// A future that asynchronously serializes something from a wrapped AsyncRead and then returns
/// the wrapped AsyncRead, the deserialized value, and how many bytes were read.
pub trait AsyncDeserialize<R: AsyncRead, S, E>
    : Future<Item = (R, S, usize), Error = (R, DeserializeError<E>)> {
    /// Consume a reader to create an `AsyncDeserialize`.
    fn from_reader(reader: R) -> Self;

    /// Return how many bytes have already been read.
    fn already_read(&self) -> usize;
}

/// An error that occured during deserialization.
#[derive(Debug)]
pub enum DeserializeError<E> {
    /// An error propagated from the underlying reader.
    ReaderError(FutIoErr),
    /// An error describing why the read data could not be deserialized into a value.
    DataError(E),
}

impl<E: Display> Display for DeserializeError<E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            DeserializeError::ReaderError(ref err) => {
                write!(f, "Deserialize reader error: {}", err)
            }
            DeserializeError::DataError(ref err) => write!(f, "Deserialize data error: {}", err),
        }
    }
}

impl<E: Error> Error for DeserializeError<E> {
    fn description(&self) -> &str {
        match *self {
            DeserializeError::ReaderError(ref err) => err.description(),
            DeserializeError::DataError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DeserializeError::ReaderError(ref err) => Some(err),
            DeserializeError::DataError(ref err) => Some(err),
        }
    }
}

impl<E> From<FutIoErr> for DeserializeError<E> {
    fn from(err: FutIoErr) -> DeserializeError<E> {
        DeserializeError::ReaderError(err)
    }
}
