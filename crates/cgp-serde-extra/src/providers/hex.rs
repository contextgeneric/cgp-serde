use alloc::string::String;
use core::fmt::Display;

use cgp::prelude::*;
use cgp_serde::components::{
    CanDeserializeValue, CanSerializeValue, ValueDeserializer, ValueDeserializerComponent,
    ValueSerializer, ValueSerializerComponent,
};
use hex::{FromHex, ToHex};
use serde::de::Error;

pub struct SerializeHex;

#[cgp_impl(SerializeHex)]
#[uses(CanSerializeValue<String>)]
impl<Value> ValueSerializer<Value>
where
    Value: ToHex,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = value.encode_hex::<String>();
        self.serialize(&str_value, serializer)
    }
}

#[cgp_impl(SerializeHex)]
#[uses(CanDeserializeValue<'de, String>)]
impl<'de, Value> ValueDeserializer<'de, Value>
where
    Value: FromHex<Error: Display>,
{
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str_value = self.deserialize(deserializer)?;
        Value::from_hex(str_value).map_err(Error::custom)
    }
}
