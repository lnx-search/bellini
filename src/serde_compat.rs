use std::fmt;

use serde::de::value::SeqAccessDeserializer;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::core::{Bytes, Document, Text, Value};

impl<'de: 'a, 'a> Deserialize<'de> for Value<'a> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .write_str("a valid JSON object (null, str, int, object, array)")
            }

            #[inline]
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
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
                Ok(Value::String(Text::from(v.to_owned())))
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(Value::String(Text::from(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Value::String(Text::from(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Bytes(Bytes::from(v.to_owned())))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Bytes(Bytes::from(v)))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                match <TypedVec as Deserialize>::deserialize(SeqAccessDeserializer::new(
                    visitor,
                )) {
                    Ok(TypedVec::String(values)) => Ok(Value::ArrayString(values)),
                    Ok(TypedVec::U64(values)) => Ok(Value::ArrayU64(values)),
                    Ok(TypedVec::I64(values)) => Ok(Value::ArrayI64(values)),
                    Ok(TypedVec::F64(values)) => Ok(Value::ArrayF64(values)),
                    Ok(TypedVec::Bool(values)) => Ok(Value::ArrayBool(values)),
                    Ok(TypedVec::Dynamic(values)) => Ok(Value::ArrayDynamic(values)),
                    Ok(TypedVec::Bytes(values)) => Ok(Value::ArrayBytes(values)),
                    Err(e) => Err(e),
                }
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
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

impl<'de: 'a, 'a> Deserialize<'de> for Text<'a> {
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
                Ok(Text::from(v.to_owned()))
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(Text::from(v))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Text::from(v))
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }
        }

        deserializer.deserialize_str(ValuesVisitor)
    }
}

impl<'de> Deserialize<'de> for Bytes {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValuesVisitor;

        impl<'de> Visitor<'de> for ValuesVisitor {
            type Value = Bytes;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON object")
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Bytes::from(v.to_owned()))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Bytes::from(v))
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
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
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
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

#[derive(Deserialize)]
#[serde(untagged)] // This sucks, but we cant really do anything about it.
pub enum TypedVec<'a> {
    #[serde(bound(deserialize = "'de: 'a"))]
    String(Vec<Text<'a>>),
    U64(Vec<u64>),
    I64(Vec<i64>),
    F64(Vec<f64>),
    Bool(Vec<bool>),
    Bytes(Vec<Bytes>),
    #[serde(bound(deserialize = "'de: 'a"))]
    Dynamic(Vec<Value<'a>>),
}
