//! The core serialization types
//!
//! These are the specialised core primitives when serializing data,
//! these can apply additional optimisations provided by rkyv when
//! working with a concrete type.

use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::time::Duration;

use rkyv::{Archive, Deserialize, Serialize};

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Default, PartialEq, Debug)]
#[archive_attr(repr(C))]
#[cfg_attr(any(feature = "validation", test), archive(check_bytes))]
/// A wrapper around a given set of document object keys and values.
///
/// The document has a specialised ID field but this is not set by default.
pub struct Document {
    id: u64,
    fields: Vec<(Text, Value)>,
}

impl Document {
    /// Create a new document with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id: 0,
            fields: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    /// The unique ID of the document.
    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline]
    /// Sets the unique ID of the document.
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    #[inline]
    /// Consume the document returning the inner values.
    pub fn into_fields(self) -> Vec<(Text, Value)> {
        self.fields
    }

    #[inline]
    /// Get a reference to the document fields.
    pub fn fields(&self) -> &[(Text, Value)] {
        &self.fields
    }

    #[inline]
    /// Get a mutable reference to the document fields.
    pub fn fields_mut(&mut self) -> &mut [(Text, Value)] {
        &mut self.fields
    }

    #[inline]
    /// Insert a new entry in the doc.
    pub fn insert(&mut self, key: impl Into<Text>, value: Value) {
        self.fields.push((key.into(), value));
    }
}

impl ArchivedDocument {
    #[inline]
    /// The unique ID of the document.
    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline]
    /// Get a reference to the document fields.
    pub fn fields(&self) -> &[(ArchivedText, ArchivedValue)] {
        &self.fields
    }
}

impl From<Vec<(Text, Value)>> for Document {
    fn from(value: Vec<(Text, Value)>) -> Self {
        Self {
            id: 0,
            fields: value,
        }
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
#[cfg_attr(any(feature = "validation", test), archive(check_bytes))]
#[cfg_attr(
    any(feature = "validation", test),
    archive_attr(check_bytes(
        bound = "__C: rkyv::validation::ArchiveContext, <__C as rkyv::Fallible>::Error: rkyv::bytecheck::Error"
    ))
)]
/// A document field value.
///
/// There are various specialisations applied for stricter types i.e. arrays of
/// all the same type, strings, etc...
pub enum Value {
    /// A null value.
    Null,
    /// A boolean value.
    Bool(bool),
    /// A UTF-8 string value.
    String(Text),
    /// A bytes value.
    Bytes(Bytes),
    /// A u64 value.
    U64(u64),
    /// A i64 value.
    I64(i64),
    /// A f64 value.
    F64(f64),
    /// A date duration offset starting from `UNIX_EPOCH` in microseconds.
    Date(i64),
    /// An array of boolean values.
    ArrayBool(#[with(rkyv::with::CopyOptimize)] Vec<bool>),
    /// An array of UTF-8 string values.
    ArrayString(Vec<Text>),
    /// An array of bytes values.
    ArrayBytes(Vec<Bytes>),
    /// An array of u64 values.
    ArrayU64(#[with(rkyv::with::Raw)] Vec<u64>),
    /// An array of i64 values.
    ArrayI64(#[with(rkyv::with::Raw)] Vec<i64>),
    /// An array of f64 values.
    ArrayF64(#[with(rkyv::with::Raw)] Vec<f64>),
    /// An array of date offset values from `UNIX_EPOCH` in microseconds.
    ArrayDate(Vec<i64>),
    /// An array of dynamic values.
    ///
    /// This is much less performant than using
    /// concrete types, this is only for supporting
    /// the JSON spec.
    ArrayDynamic(
        #[omit_bounds]
        #[cfg_attr(any(feature = "validation", test), archive_attr(omit_bounds))]
        Vec<Value>,
    ),
    /// An object of string keys and dynamic values.
    Object(
        #[omit_bounds]
        #[cfg_attr(any(feature = "validation", test), archive_attr(omit_bounds))]
        Vec<(Text, Value)>,
    ),
}

impl Value {
    /// Get the string representation of the enum type.
    pub fn as_type(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Bytes(_) => "bytes",
            Value::U64(_) => "u64",
            Value::I64(_) => "i64",
            Value::F64(_) => "f64",
            Value::Date(_) => "datetime",
            Value::ArrayBool(_) => "array<bool>",
            Value::ArrayString(_) => "array<string>",
            Value::ArrayBytes(_) => "array<bytes>",
            Value::ArrayU64(_) => "array<u64>",
            Value::ArrayI64(_) => "array<i64>",
            Value::ArrayF64(_) => "array<f64>",
            Value::ArrayDate(_) => "array<datetime>",
            Value::ArrayDynamic(_) => "array<any>",
            Value::Object(_) => "object",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_type())
    }
}

macro_rules! derive_from {
    ($t:ty, $variant:ident) => {
        impl From<$t> for Value {
            #[inline]
            fn from(v: $t) -> Self {
                Self::$variant(v.into())
            }
        }
    };
}

derive_from!(bool, Bool);
derive_from!(u64, U64);
derive_from!(i64, I64);
derive_from!(f64, F64);
derive_from!(Text, String);
derive_from!(String, String);
derive_from!(&str, String);
derive_from!(Bytes, Bytes);
derive_from!(Vec<u8>, Bytes);
derive_from!(Vec<bool>, ArrayBool);
derive_from!(Vec<u64>, ArrayU64);
derive_from!(Vec<i64>, ArrayI64);
derive_from!(Vec<f64>, ArrayF64);
derive_from!(Vec<Text>, ArrayString);
derive_from!(Vec<Bytes>, ArrayBytes);

macro_rules! write_array {
    ($f:expr, $values:expr) => {{
        write!($f, "[")?;
        for (i, value) in $values.iter().enumerate() {
            if i == 0 {
                write!($f, "{value}")?;
            } else {
                write!($f, ", {value}")?;
            }
        }
        write!($f, "]")
    }};
    ($f:expr, $values:expr, debug) => {{
        write!($f, "[")?;
        for (i, value) in $values.iter().enumerate() {
            if i == 0 {
                write!($f, "{value:?}")?;
            } else {
                write!($f, ", {value:?}")?;
            }
        }
        write!($f, "]")
    }};
}

impl Debug for ArchivedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchivedValue::Null => write!(f, "null"),
            ArchivedValue::Bool(v) => write!(f, "{v}"),
            ArchivedValue::String(v) => write!(f, "{v:?}"),
            ArchivedValue::Bytes(v) => write!(f, "{v:?}"),
            ArchivedValue::U64(v) => write!(f, "{v}"),
            ArchivedValue::I64(v) => write!(f, "{v}"),
            ArchivedValue::F64(v) => write!(f, "{v}"),
            ArchivedValue::Date(v) => write!(f, "{v:?}"),
            ArchivedValue::ArrayBool(values) => write_array!(f, values),
            ArchivedValue::ArrayString(values) => write_array!(f, values, debug),
            ArchivedValue::ArrayBytes(values) => write_array!(f, values, debug),
            ArchivedValue::ArrayU64(values) => write_array!(f, values),
            ArchivedValue::ArrayI64(values) => write_array!(f, values),
            ArchivedValue::ArrayF64(values) => write_array!(f, values),
            ArchivedValue::ArrayDate(values) => write_array!(f, values, debug),
            ArchivedValue::ArrayDynamic(values) => write_array!(f, values, debug),
            ArchivedValue::Object(object) => {
                write!(f, "{{")?;
                for (i, (key, value)) in object.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{key:?}: {value:?}")?;
                    } else {
                        write!(f, ", {key:?}: {value:?}")?;
                    }
                }
                write!(f, "}}")
            },
        }
    }
}

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Eq, PartialEq)]
#[archive_attr(repr(C))]
#[cfg_attr(any(feature = "validation", test), archive(check_bytes))]
/// A UTF-8 encoded string.
pub struct Text(#[with(rkyv::with::Raw)] Vec<u8>);

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value.into_bytes())
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl Deref for ArchivedText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl AsRef<str> for ArchivedText {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", <Self as AsRef<str>>::as_ref(self))
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as AsRef<str>>::as_ref(self))
    }
}

impl Debug for ArchivedText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl Display for ArchivedText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[archive_attr(repr(C), derive(Debug))]
#[cfg_attr(any(feature = "validation", test), archive(check_bytes))]
/// An arbitrary byte slice backed by a `Cow`
pub struct Bytes(#[with(rkyv::with::Raw)] Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
