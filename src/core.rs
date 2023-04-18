//! The core serialization types
//!
//! These are the specialised core primitives when serializing data,
//! these can apply additional optimisations provided by rkyv when
//! working with a concrete type.

use std::borrow::Cow;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Deref;

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Default)]
#[archive_attr(derive(CheckBytes), repr(C))]
/// Context associated with a document layout.
pub struct DocContext {
    #[with(rkyv::with::AsVec)]
    /// The field ID -> field name mapping.
    pub field_lookup: BTreeMap<u32, String>,
}


#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Default)]
#[archive_attr(repr(C))]
pub struct Document<'a>(Vec<(Text<'a>, Value<'a>)>);

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

#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
pub enum Value<'a> {
    Null,
    Bool(bool),
    String(Text<'a>),
    Bytes(Bytes<'a>),
    U64(u64),
    I64(i64),
    F64(f64),
    ArrayBool(#[with(rkyv::with::CopyOptimize)] Vec<bool>),
    ArrayString(Vec<Text<'a>>),
    ArrayBytes(Vec<Bytes<'a>>),
    ArrayU64(#[with(rkyv::with::Raw)] Vec<u64>),
    ArrayI64(#[with(rkyv::with::Raw)] Vec<i64>),
    ArrayF64(#[with(rkyv::with::Raw)] Vec<f64>),
    ArrayDynamic(
        #[omit_bounds]
        Vec<Value<'a>>
    ),
    Object(
        #[omit_bounds]
        Vec<(Text<'a>, Value<'a>)>,
    )
}


#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(repr(C))]
pub struct Text<'a>(
    #[with(rkyv::with::AsOwned)]
    // #[with(rkyv::with::Raw)]
    pub Cow<'a, [u8]>,
);


#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(repr(C))]
pub struct Bytes<'a>(
    #[with(rkyv::with::AsOwned)]
    // #[with(rkyv::with::Raw)]
    pub Cow<'a, [u8]>,
);
