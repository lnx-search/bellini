use std::borrow::Cow;
use rkyv::{Archive, Serialize, Deserialize};
use smallvec::SmallVec;

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

#[repr(C)]
#[derive(Archive, Serialize, Deserialize, Default)]
#[archive_attr(repr(C))]
pub struct Document<'a>(Vec<(Text<'a>, Value<'a>)>);

fn main() -> anyhow::Result<()> {
    let val = Value::String(Text(Cow::Borrowed("Hello".as_ref())));
    let doc = Document(vec![(Text(Cow::Borrowed("bonk".as_ref())), val)]);

    let data = rkyv::to_bytes::<_, 1024>(&doc)?;
    dbg!(data.len());

    Ok(())
}
