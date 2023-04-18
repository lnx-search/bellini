use std::borrow::Cow;
use std::fmt;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};

use crate::core::{Bytes, Text, Value};
use crate::Document;


impl<'de> Deserialize<'de> for Value<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a valid JSON object (null, str, int, object, array)")
            }

            #[inline]
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
                Ok(Value::Bool(v))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Value::I64(v))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Value::U64(v))
            }

            #[inline]
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(Value::F64(v))
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Value::String(Text(Cow::Owned(v.as_bytes().to_owned()))))
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(Value::String(Text(Cow::Borrowed(v.as_bytes()))))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Value::String(Text(Cow::from(v.into_bytes()))))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
                Ok(Value::Bytes(Bytes(Cow::from(v.to_vec()))))
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> where E: Error {
                Ok(Value::Bytes(Bytes(Cow::Borrowed(v))))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: Error {
                Ok(Value::Bytes(Bytes(Cow::Owned(v))))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E> where E: Error {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where V: SeqAccess<'de> {
                let mut vec = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::ArrayDynamic(vec))
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>
            {
                let mut values = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some((key, value)) = visitor.next_entry()? {
                    values.push((key, value));
                }

                Ok(Value::Object(values))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}


impl<'de> Deserialize<'de> for Text<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {

        struct ValuesVisitor;

        impl<'de> Visitor<'de> for ValuesVisitor {
            type Value = Text<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON object")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Text(Cow::Owned(v.as_bytes().to_owned())))
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(Text(Cow::Borrowed(v.as_bytes())))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Text(Cow::from(v.into_bytes())))
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>
            {
                Deserialize::deserialize(deserializer)
            }

        }

        deserializer.deserialize_str(ValuesVisitor)
    }
}

impl<'de> Deserialize<'de> for Document<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct ValuesVisitor;

        impl<'de> Visitor<'de> for ValuesVisitor {
            type Value = Document<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON object")
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>
            {
                let mut values = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some((key, value)) = visitor.next_entry()? {
                    values.push((key, value));
                }

                Ok(Document::from(values))
            }
        }

        deserializer.deserialize_map(ValuesVisitor)
    }
}