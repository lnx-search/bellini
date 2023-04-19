mod core;
mod decoder;
mod encoder;
mod serializer;

#[cfg(feature = "serde")]
mod serde_compat;

#[cfg(feature = "utils")]
pub use decoder::BufferWalker;
pub use decoder::{ArchivedIterator, Archiver, Decoder, UnsafeArchiver, FOOTER_SIZE};
#[cfg(feature = "validation")]
pub use decoder::{CheckedArchiver, DeserializerIterator};
#[cfg(feature = "utils")]
pub use encoder::ChecksumAndLenWriter;
pub use encoder::{Encoder, DEFAULT_SCRATCH_SPACE};

pub use self::core::{
    ArchivedBytes,
    ArchivedDocument,
    ArchivedText,
    ArchivedValue,
    Bytes,
    Document,
    Text,
    Value,
};
