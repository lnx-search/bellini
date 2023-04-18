use std::borrow::Cow;
use rkyv::{Archive, Serialize, Deserialize};

#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]
pub enum Value<'a> {
    String(Text<'a>),
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
