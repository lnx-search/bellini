mod core;
mod encoder;
mod serializer;

#[cfg(feature = "serde")]
mod serde_compat;

pub use encoder::{Encoder, DEFAULT_SCRATCH_SPACE};

pub use self::core::{Bytes, Document, Text, Value};
