//! The core serialization types
//!
//! These are the specialised core primitives when serializing data,
//! these can apply additional optimisations provided by rkyv when
//! working with a concrete type.

use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use rkyv::{Archive, Deserialize, Serialize};

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Default)]
#[archive_attr(repr(C))]
#[cfg_attr(feature = "validation", archive(check_bytes))]
/// A wrapper around a given set of document object keys and values.
pub struct Document<'a>(Vec<(Text<'a>, Value<'a>)>);

impl<'a> Document<'a> {
    /// Create a new document with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Consume the document returning the inner values.
    pub fn into_inner(self) -> Vec<(Text<'a>, Value<'a>)> {
        self.0
    }

    /// Insert a new entry in the doc.
    pub fn insert(&mut self, key: impl Into<String>, value: Value<'a>) {
        self.0.push((Text::from(key.into()), value));
    }
}

impl<'a> From<Vec<(Text<'a>, Value<'a>)>> for Document<'a> {
    fn from(value: Vec<(Text<'a>, Value<'a>)>) -> Self {
        Self(value)
    }
}

impl<'a> Deref for Document<'a> {
    type Target = Vec<(Text<'a>, Value<'a>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Document<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
#[cfg_attr(feature = "validation", archive(check_bytes))]
/// A document field value.
///
/// There are various specialisations applied for stricter types i.e. arrays of
/// all the same type, strings, etc...
pub enum Value<'a> {
    /// A null value.
    Null,
    /// A boolean value.
    Bool(bool),
    /// A UTF-8 string value.
    String(Text<'a>),
    /// A bytes value.
    Bytes(Bytes<'a>),
    /// A u64 value.
    U64(u64),
    /// A i64 value.
    I64(i64),
    /// A f64 value.
    F64(f64),
    /// An array of boolean values.
    ArrayBool(#[with(rkyv::with::CopyOptimize)] Vec<bool>),
    /// An array of UTF-8 string values.
    ArrayString(Vec<Text<'a>>),
    /// An array of bytes values.
    ArrayBytes(Vec<Bytes<'a>>),
    /// An array of u64 values.
    ArrayU64(#[with(rkyv::with::Raw)] Vec<u64>),
    /// An array of i64 values.
    ArrayI64(#[with(rkyv::with::Raw)] Vec<i64>),
    /// An array of f64 values.
    ArrayF64(#[with(rkyv::with::Raw)] Vec<f64>),
    /// An array of dynamic values.
    ///
    /// This is much less performant than using
    /// concrete types, this is only for supporting
    /// the JSON spec.
    ArrayDynamic(#[omit_bounds] Vec<Value<'a>>),
    /// An object of string keys and dynamic values.
    Object(#[omit_bounds] Vec<(Text<'a>, Value<'a>)>),
}

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(repr(C))]
#[cfg_attr(feature = "validation", archive(check_bytes))]
/// A UTF-8 encoded string.
pub struct Text<'a>(#[with(rkyv::with::AsOwned)] Cow<'a, [u8]>);

impl<'a> From<&'a str> for Text<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::Borrowed(value.as_ref()))
    }
}

impl<'a> From<String> for Text<'a> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value.into_bytes()))
    }
}

impl<'a> AsRef<str> for Text<'a> {
    fn as_ref(&self) -> &str {
        // SAFETY:
        // The string is guaranteed to be UTF8 going into the type.
        unsafe { std::str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(repr(C))]
#[cfg_attr(feature = "validation", archive(check_bytes))]
pub struct Bytes<'a>(#[with(rkyv::with::AsOwned)] Cow<'a, [u8]>);

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> From<Vec<u8>> for Bytes<'a> {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> AsRef<[u8]> for Text<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
