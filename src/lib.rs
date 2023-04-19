mod core;
mod decoder;
mod encoder;
mod serializer;

#[cfg(feature = "serde")]
mod serde_compat;

pub use decoder::{ArchivedIterator, Archiver, Decoder, UnsafeArchiver};
#[cfg(feature = "validation")]
pub use decoder::{CheckedArchiver, DeserializerIterator};
pub use encoder::{ChecksumAndLenWriter, Encoder, DEFAULT_SCRATCH_SPACE};

pub use self::core::{Bytes, Document, Text, Value};
