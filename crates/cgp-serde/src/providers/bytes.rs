use core::fmt::Display;

use cgp::prelude::*;
use serde::de::{Error, Visitor};

use crate::components::{
    ValueDeserializer, ValueDeserializerComponent, ValueSerializer, ValueSerializerComponent,
};

pub struct SerializeBytes;

#[cgp_impl(SerializeBytes)]
impl<Value> ValueSerializer<Value>
where
    Value: AsRef<[u8]>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(value.as_ref())
    }
}

#[cgp_impl(SerializeBytes)]
impl<'a, Value> ValueDeserializer<'a, Value>
where
    Value: From<&'a [u8]>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let bytes = deserializer.deserialize_bytes(SerializeBytes)?;
        Ok(bytes.into())
    }
}

#[cgp_impl(new TryDeserializeBytes)]
impl<'a, Value> ValueDeserializer<'a, Value>
where
    Value: TryFrom<&'a [u8], Error: Display>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let bytes = deserializer.deserialize_bytes(SerializeBytes)?;
        let value = bytes.try_into().map_err(D::Error::custom)?;

        Ok(value)
    }
}

impl<'a> Visitor<'a> for SerializeBytes {
    type Value = &'a [u8];

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        formatter.write_str("bytes")
    }

    fn visit_borrowed_bytes<E>(self, bytes: &'a [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(bytes)
    }
}
